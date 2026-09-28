enum Estado {
    INICIO,
    INTEIRO,
    PONTO,
    DECIMAL,
    ERRO,
}

struct Automato {
    estado: Estado,
}
