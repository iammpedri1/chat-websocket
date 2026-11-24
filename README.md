# Chat WebSocket

## O que construí?
* Um servidor WebSocket capaz de lidar com múltiplas conexões simultâneas.
* Salas de bate-papo nomeadas com distribuição de mensagens via **Pub/Sub**.
* Rastreador de presença de usuários (notificações de entrada e saída).

## O que é Pub/Sub?
É um mecanismo de mensageria que entrega a mensagem enviada por um usuário para todos os outros que estão na mesma sala, sem que quem recebe precise saber diretamente quem a enviou.

## Como o projeto funciona?
* O servidor é desenvolvido em **Rust** e utiliza o **Tokio** para gerenciar várias conexões assíncronas ao mesmo tempo.
* Cada pessoa que entra no chat permanece conectada via **WebSocket**.
* As mensagens são distribuídas por um canal `broadcast`, que atua no modelo *Pub/Sub*.
* Por enquanto, existe apenas uma sala global (os apelidos não criam salas separadas).

## O que dá para fazer?
* Escolher um apelido para entrar no chat (apelidos repetidos são recusados).
* Enviar mensagens de até **500 caracteres**.
* Usar o comando `/usuarios` para ver quem está conectado no momento.
* Usar o comando `/sair` para desconectar do chat.

## Como executar?
1. Abra o terminal na pasta raiz do projeto e execute:
   ```bash
   cargo run
   ```
2. O servidor iniciará por padrão no endereço `127.0.0.1:8080`.
3. Abra o arquivo `src/frontend/index.html` no seu navegador.
4. A interface se conectará automaticamente ao servidor pelo endereço `ws://127.0.0.1:8080`.

> **Nota:** Para alterar o endereço ou a porta, configure a variável de ambiente `CHAT_ADDR` e atualize a URL do WebSocket no arquivo `index.html`.

## Estrutura de pastas
* `src/main.rs` — Contém a lógica do servidor, o gerenciamento de conexões e o estado do chat.
* `src/frontend/index.html` — Contém a estrutura da interface e a comunicação com o WebSocket.
* `src/frontend/style.css` — Contém os estilos visuais da página.
* `Cargo.toml` — Lista as dependências e configurações do projeto Rust.
* `Cargo.lock` — Guarda as versões exatas das dependências instaladas.
