## Chat webSocket 

> o que construi?
# um servidor websocket que pode lidae com multíplas conexões simultâneas
# as sala de bate-papo são nomeadas com distribuição de mensagems pub/sub
# rastreador de presença do usuário(notificações> entrada/saida)

> o que é pub/sub
# é um tipo de mecanismo que entrega a mensagem enviada por um usuário para todas as outras que estão na mesma sala,sem que o usuario que ira receber precise saber quem enviou

> como o projeto funciona?
# o servidor é feito em rust e usa tokio para cuidar de várias conexões ao mesmo tempo
# cada pessoa que entra no chat fica conectada por websocket
# as mensagens são distribuídas pelo canal broadcast, que funciona como pub/sub
# por enquanto existe uma sala global; os apelidos não criam salas separadas

> o que dá pra fazer?
# escolher um apelido para entrar no chat
# enviar mensagens de até 500 caracteres
# usar /usuarios para ver quem está conectado
# usar /sair para sair do chat
# apelidos repetidos são recusados

> como executar?
# abra o terminal na pasta do projeto e rode: `cargo run`
# o servidor inicia por padrão em `127.0.0.1:8080`
# depois abra `src/frontend/index.html` no navegador
# o site conecta ao servidor pelo endereço `ws://127.0.0.1:8080`
# para mudar o endereço, configure `CHAT_ADDR` e atualize o websocket no `index.html`

> como as pastas estão organizadas?
# `src/main.rs` contém o servidor, as conexões e o estado do chat
# `src/frontend/index.html` contém a tela e a comunicação com o websocket
# `src/frontend/style.css` contém os estilos do site
# `Cargo.toml` lista as dependências usadas pelo projeto
# `Cargo.lock` guarda as versões resolvidas dessas dependências
