## Introdução
Esse projeto foi feito pra um server de minecraft de um amigo (@flipbit03)
A ideia é um bot do discord que response o status atual do servidor usando comandos.

## Como rodar o projeto
Versão do Rust usada no projeto: 1.77.2

- Tenha o Rust instalado na sua máquina
- Crie um arquivo `.env` na raiz do projeto e popule ele com as variáveis baseado no arquivo pré-existente chamado `.env.template`. Você também pode usar as variáveis diretamente no seu ambiente, sem usar o arquivo `.env`
- Execute o projeto utilizando `cargo run` ou buildando e executando o projeto.

## Env vars
- GUILD_ID -> Id do server do discord que o bot irá registrar os comandos.
- DISCORD_TOKEN -> Token do seu bot
- SERVER_IP -> IP do servidor de minecraft, ex: "localhost"
- SERVER_PORT -> Porta do server de minecraft (Default: 25565)