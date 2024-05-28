pub mod app;

use crate::app::tests::lista_tarefas_test;
use app::models::tarefas::*;

fn main() {
    let work = tarefa::Tarefa {
        descricao: String::from("A1"),
        concluida: false,
    };
    let work2 = tarefa::Tarefa {
        descricao: String::from("A2"),
        concluida: true,
    };
    let work3 = tarefa::Tarefa {
        descricao: String::from("A3"),
        concluida: false,
    };

    let mut lista = lista_tarefas::TarefasList {
        tarefas: vec![work, work2, work3],
    };

    lista.nova_tarefa(String::from("A4"));
    lista.remover_tarefa(3);
    lista.nova_tarefa(String::from("A5"));
    lista.marcar_concluido(3);

    lista.imprimir_tarefas();

    println!("OUTROOO TESTEEE\n");

    let resultado_teste = lista_tarefas_test::testar_new();
    println!("Resultado do teste: {:#?}", resultado_teste);
}
