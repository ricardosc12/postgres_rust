### Performance
Implementar Postgres em Rust com SQLX pode não ser uma escolha ideal quando se considera o desempenho. 
Realizei testes de estresse e constatei que ele não suporta sequer 1000 rps. Além disso, alterar os parâmetros do PostgreSQL faz com que o tempo de resposta aumente sem razão aparente. 
Mais detalhes podem ser encontrados neste link do StackOverflow:

[Increasing-shared-memory-results-in-increased-response-time-performance-issue](https://stackoverflow.com/questions/78562464/increasing-shared-memory-results-in-increased-response-time-performance-issue)

### Indexs

Índices no PostgreSQL são criados automaticamente quando uma coluna na tabela é definida como chave primária ou possui uma restrição de exclusividade (UNIQUE). 

Um teste comparativo na pasta stress_test mostra a diferença na busca por campos que não são indexados automaticamente, como o caso do campo "nome", e a diferença quando criamos um índice para o mesmo.
