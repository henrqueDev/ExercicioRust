use super::super::models::tarefas::*;

pub fn testar_new() -> bool {
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
        tarefas: vec![work],
    };

    lista.new(vec![work2, work3]);

    let work2 = tarefa::Tarefa {
        descricao: String::from("A2"),
        concluida: true,
    };
    let work3 = tarefa::Tarefa {
        descricao: String::from("A3"),
        concluida: false,
    };

    let vector2 = vec![work2, work3];
    //assert_eq!(lista.tarefas, vector2);
    /*let result = lista.tarefas.iter().zip(vector2.iter()).all(|(a, b)| {
        if a.descricao == b.descricao && a.concluida == b.concluida {
            return true;
        } else {
            return false;
        }
    });*/
    /*assert_eq!(
        lista.tarefas, vector2,
        "we are testing addition with {:#?} and {:#?}",
        lista.tarefas, vector2
    );*/
    let boleano = lista.tarefas == vector2;
    boleano
}
