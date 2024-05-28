use super::tarefa::Tarefa;

pub struct TarefasList {
    pub tarefas: Vec<Tarefa>,
}

impl TarefasList {
    pub fn new(&mut self, tarefas: Vec<Tarefa>) {
        let tarefas_list = TarefasList { tarefas };
        self.tarefas = tarefas_list.tarefas;
    }

    pub fn nova_tarefa(&mut self, tarefa_descricao: String) -> &Vec<Tarefa> {
        let new_tarefa = Tarefa {
            descricao: tarefa_descricao,
            concluida: false,
        };
        self.tarefas.push(new_tarefa);
        let updated_list = &self.tarefas;
        updated_list
    }

    pub fn remover_tarefa(&mut self, indice: usize) -> &Vec<Tarefa> {
        self.tarefas.remove(indice);
        let updated_list = &self.tarefas;
        updated_list
    }

    pub fn marcar_concluido(&mut self, indice: usize) -> &Vec<Tarefa> {
        if let Some(tarefa) = self.tarefas.get_mut(indice) {
            tarefa.concluida = true;
        }
        &self.tarefas
    }

    pub fn imprimir_tarefas(&mut self) {
        for tarefa in &self.tarefas {
            println!(
                "Descricao: {descript} \n Concluida: {concluida}",
                descript = tarefa.descricao,
                concluida = tarefa.concluida
            );
        }
    }
}
