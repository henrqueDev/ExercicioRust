Crie uma lista de tarefas

Neste exercício, você implementará uma lista de tarefas simples usando Rust. A lista deve permitir aos usuários adicionar, remover e marcar tarefas como concluídas.

Tarefas:

Defina uma struct Tarefa para armazenar informações sobre uma única tarefa. A struct deve ter os seguintes campos:

descricao: String - Uma breve descrição da tarefa.
concluida: bool - Um valor booleano indicando se a tarefa foi concluída (true) ou não (false).
Defina uma struct ListaDeTarefas para gerenciar a coleção de tarefas. A struct deve ter um campo:

tarefas: Vec<Tarefa> - Um vetor para armazenar as Tarefas.
Implemente as seguintes funções na struct ListaDeTarefas:

nova_tarefa: Essa função deve receber uma descrição da tarefa (String) como entrada e adicionar uma nova Tarefa à lista de tarefas.
remover_tarefa: Essa função deve receber o índice da tarefa (usize) como entrada e remover a tarefa correspondente da lista.
marcar_concluida: Essa função deve receber o índice da tarefa (usize) como entrada e marcar a tarefa correspondente como concluída (alterar o valor do campo concluida para true).
imprimir_tarefas: Essa função deve iterar sobre a lista de tarefas e imprimir cada tarefa junto com seu status (concluída ou não concluída) em um formato legível.