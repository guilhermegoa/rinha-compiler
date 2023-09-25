# rinha-interpreter

Esta é a minha primeira mexendo com Rust, e devo dizer que foi uma experiência diferente e desafiadora. Neste projeto, explorei as maravilhas da Rust e enfrentei o desafio de construir um interpretador.Além disso, durante o desenvolvimento deste interpretador em Rust, gastei muito tempo na documentação oficial da linguagem e também passei horas pesquisando e analisando código semelhante de projetos existentes.

## Desafio Inspirado

O desafio original pode ser encontrado no GitHub [aqui](https://github.com/aripiprazole/rinha-de-compiler/tree/main). 

## Executando

Para executar o interpretador e analisar um exemplo JSON, siga os passos abaixo:

1. Certifique-se de ter as dependências do projeto instaladas. Caso ainda não tenha instalado o Rust e o Cargo, você pode fazê-lo seguindo as instruções em [Rust's website](https://www.rust-lang.org/).

2. Clone este repositório para o seu sistema, se ainda não o fez, usando o seguinte comando:

```bash
git clone https://github.com/seu-usuario/seu-projeto.git
```

3. Navegue até a pasta do projeto:

```bash
cd seu-projeto
```

3. Para executar o interpretador e analisar um exemplo JSON, você pode usar o seguinte comando:

```bash
cargo run -- examples/<xxx>.json
```

### Executando no Docker

Se preferir, você também pode executar este projeto em um contêiner Docker. Certifique-se de ter o Docker instalado em seu sistema antes de seguir estas etapas:

1. Clone este repositório para o seu sistema, se ainda não o fez:

```bash
git clone https://github.com/seu-usuario/seu-projeto.git
```

2. Navegue até a pasta do projeto:

```bash
cd seu-projeto
```
3. Construa o projeto:

```bash
docker build -t rinha .
```

4. Rodeo projeto:

```bash
docker run -v $(pwd)/examples:/app/examples nome-da-imagem cargo run -- examples/<xxx>.json
```


### Exemplos disponíveis

- print
- sum
- tuple
- combination
- fib

## Conclusão

Desenvolver este interpretador em Rust foi uma jornada empolgante e enriquecedora.
Através da pesquisa, aprendizado e experimentação, conseguimos criar uma ferramenta que pode analisar e interpretar dados em formato

Ps.: Fiquei satisfeito com o resultado alcançado, apesar de entender que ainda há espaço para melhorias.