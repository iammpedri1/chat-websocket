use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_tungstenite::tungstenite::Message;

#[derive(Debug, Clone)]
pub struct MensagemChat {
    pub remetente: String,
    pub conteudo: String,
    pub timestamp: String,
}

impl MensagemChat {
    fn formatada(&self) -> String {
        format!("[{}] {}: {}", self.timestamp, self.remetente, self.conteudo)
    }
}

pub struct EstadoChat {
    pub transmissor: broadcast::Sender<MensagemChat>,
    pub usuarios: Mutex<HashMap<String, String>>,
}

impl EstadoChat {
    fn novo() -> Arc<Self> {
        let (transmissor, _) = broadcast::channel(1000);
        Arc::new(EstadoChat {
            transmissor,
            usuarios: Mutex::new(HashMap::new()),
        })
    }

    fn adicionar_usuario(&self, endereco: &str, apelido: &str) -> Result<bool, &'static str> {
        let mut usuarios = self
            .usuarios
            .lock()
            .map_err(|_| "estado de usuários indisponível")?;

        if usuarios
            .values()
            .any(|nome| nome.eq_ignore_ascii_case(apelido))
        {
            return Ok(false);
        }

        usuarios.insert(endereco.to_string(), apelido.to_string());
        Ok(true)
    }

    fn remover_usuario(&self, endereco: &str) -> Option<String> {
        self.usuarios.lock().ok()?.remove(endereco)
    }

    fn listar_usuarios(&self) -> Vec<String> {
        self.usuarios
            .lock()
            .map(|u| u.values().cloned().collect())
            .unwrap_or_default()
    }

    fn enviar_mensagem(&self, remetente: &str, conteudo: &str) {
        let mensagem = MensagemChat {
            remetente: remetente.to_string(),
            conteudo: conteudo.to_string(),
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
        };
        let _ = self.transmissor.send(mensagem);
    }
}

async fn tratar_conexao(stream: TcpStream, endereco: String, estado: Arc<EstadoChat>) {
    let ws_stream = match tokio_tungstenite::accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            eprintln!("Erro no handshake WebSocket de {}: {}", endereco, e);
            return;
        }
    };

    println!("Nova conexão WebSocket: {}", endereco);
    let (mut escritor, mut leitor) = ws_stream.split();

    let _ = escritor
        .send(Message::Text(
            "Bem-vindo ao chat! Digite seu apelido:".into(),
        ))
        .await;

    let apelido = loop {
        let apelido = match leitor.next().await {
            Some(Ok(msg)) => {
                let texto = msg.to_text().unwrap_or("Anônimo").trim().to_string();
                if texto.is_empty() {
                    "Anônimo".to_string()
                } else {
                    texto.chars().take(20).collect()
                }
            }
            _ => return,
        };

        match estado.adicionar_usuario(&endereco, &apelido) {
            Ok(true) => break apelido,
            Ok(false) => {
                let aviso = "Esse apelido já está em uso. Escolha outro.";
                if escritor.send(Message::Text(aviso.into())).await.is_err() {
                    return;
                }
            }
            Err(erro) => {
                eprintln!("Não foi possível registrar {}: {}", endereco, erro);
                return;
            }
        }
    };

    estado.enviar_mensagem("Sistema", &format!("{} entrou no chat", apelido));

    let usuarios = estado.listar_usuarios();
    let _ = escritor
        .send(Message::Text(
            format!("Usuários online: {}", usuarios.join(", ")).into(),
        ))
        .await;

    let mut receptor = estado.transmissor.subscribe();
    let apelido_clone = apelido.clone();
    let endereco_clone = endereco.clone();

    loop {
        tokio::select! {
            msg = leitor.next() => {
                match msg {
                    Some(Ok(Message::Text(texto))) => {
                        let texto = texto.trim().to_string();
                        if texto == "/usuarios" {
                            let usuarios = estado.listar_usuarios();
                            let _ = escritor
                                .send(Message::Text(
                                    format!("Usuários online: {}", usuarios.join(", ")).into(),
                                ))
                                .await;
                        } else if texto == "/sair" {
                            break;
                        } else if !texto.is_empty() {
                            let limpo: String = texto.chars().take(500).collect();
                            estado.enviar_mensagem(&apelido_clone, &limpo);
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
                }
            }
            resultado = receptor.recv() => {
                match resultado {
                    Ok(mensagem) => {
                        if mensagem.remetente != apelido_clone {
                            if escritor
                                .send(Message::Text(mensagem.formatada().into()))
                                .await
                                .is_err()
                            {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(quantidade)) => {
                        let aviso = format!(
                            "Sistema: você perdeu {quantidade} mensagens; reconecte para sincronizar."
                        );
                        if escritor.send(Message::Text(aviso.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }
    if let Some(apelido) = estado.remover_usuario(&endereco_clone) {
        estado.enviar_mensagem("Sistema", &format!("{} saiu do chat", apelido));
    }
    println!("Conexão encerrada: {}", endereco_clone);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let estado = EstadoChat::novo();
    let endereco = std::env::var("CHAT_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".into());

    let listener = TcpListener::bind(&endereco).await?;
    println!("Servidor de chat WebSocket rodando em ws://{}", endereco);

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let estado_clone = estado.clone();
                let endereco = addr.to_string();
                tokio::spawn(async move {
                    tratar_conexao(stream, endereco, estado_clone).await;
                });
            }
            Err(e) => eprintln!("Erro ao aceitar conexão: {}", e),
        }
    }
}
