### Indexs

Indexs em Postgresql são criados automaticamente quando a coluna na tabela é definida 
como chave primária ou há uma restrição de exclusividade na coluna (UNIQUE).

Um teste comparativo na pasta stress_test mostra a diferença na buscar por campos não indexados automaticamente, como no caso de 
nome, e a diferença quando criamos index para o mesmo.

