# pas

!['se voce souber fazer uma logo legal para o rubr, sinta-se livre para contribuir no projeto! :)']()

**rubr** (_Rust_ em portugues!!!! ) permite 
uma nova forma de escrever rust em ptbr!!!

Aqui uma simples forma de usar rubr

```rust
rubr::rustrj! {
    usar std::collections::HashMap como Dicionario;

    contrato ValorChave {
        funcao salvar_valor(&proprio, chave: linguicaodebits, valor: linguicaodebits);
        funcao pegar_valor(&proprio, chave: linguicaodebits) -> Resultado<Opcao<&linguicaodebits>, linguicaodebits>;
    }

        
    estatico mutavel DICIONARIO: Opcao<Dicionario<linguicaodebits, linguicaodebits>> = Nada;

    temqueserassim HM;

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
}
```


## Outras linguagens 

- French: [rouille](https://github.com/bnjbvr/rouille)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [oxido](https://github.com/fdschonborn/oxido)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)

## Licença

[WTFPL](http://www.wtfpl.net/).
