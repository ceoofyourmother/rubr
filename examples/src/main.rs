use core::panic;

rubr::rustrj! {
    usar std::collections::HashMap como Dicionario;

    contrato ValorChave {
        funcao salvar_valor(&proprio, chave: linguicaodebits, valor: linguicaodebits);
        funcao pegar_valor(&proprio, chave: linguicaodebits) -> Resultado<Opcao<&linguicaodebits>, linguicaodebits>;
    }

        
    estatico mutavel DICIONARIO: Opcao<Dicionario<linguicaodebits, linguicaodebits>> = Nada;

    estrutura HM;

    implementacao ValorChave for HM{
        funcao salvar_valor(&proprio, chave: linguicaodebits, valor: linguicaodebits) {
            salvar c = issovaidarmerda {   
                DICIONARIO.pega_ou_inseri_em(Padrao::padrao)
            };
            c.meteisso(chave, valor);
        }
        funcao pegar_valor(&proprio, chave: linguicaodebits) -> Resultado<Opcao<&linguicaodebits>, linguicaodebits> {
            se salvar Qualquer(c) = issovaidarmerda { DICIONARIO.como_referencia() } {
                Tudocorreto(c.pegaisso(&chave))
            } senao {
                Vixi("nao achei a chave".dentro())
            }
        }
    }

    assincrono funcao exemplo (){}

    assincrono funcao exemplo2(){
        exemplo().espera;
    }

    funcao principal() {
        salvar mutavel x = 31;
        para i em 0..10 {
            salvar v = naoparanao{
                break i;
            };

        
            enquanto x < v {
                x += 1;
            };

            x = se salvar Some(r) = Some(i) {
                r
            } senao { 
                12
            };
       }
       printaisso!("{}", x);

       HM.salvar_valor(String::from("br"), String::from("rust"));

        salvar v = HM.pegar_valor(String::from("br"));

        tabatendo v {
            Tudocorreto(Qualquer(v)) => {
                printaisso!("Valor: {}", v);
            },
            Tudocorreto(Nada) => {
                printaisso!("Chave não encontrada");
            },
            Vixi(err) =>  {
                printaisso!("Erro: {}", err);
            }
        }
    }
}
