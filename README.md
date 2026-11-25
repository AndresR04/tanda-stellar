# Tanda en Stellar

Un contrato inteligente en Stellar (Soroban) que lleva el registro de una tanda.

## El problema

En Costa Rica es común hacer "tandas": un grupo de amigos, familiares o compañeros
aporta una cuota cada ronda, y en cada ronda le toca a una persona distinta recibir
el total. Hoy esto se maneja con WhatsApp y confianza. Se pierden mensajes, se
discute quién ya pagó y a quién le toca.

## Por qué elegí este problema

Es pequeño, real y cercano. No necesita reglas complicadas, pero sí necesita
transparencia: todos deben poder ver quién pagó y a quién le toca. Un contrato
inteligente da ese registro claro y que nadie puede alterar.

## Cómo funciona

- `init(admin, members, amount)`: crea la tanda con sus miembros y la cuota. Solo se puede hacer una vez.
- `pay(member)`: un miembro registra su pago de la ronda actual. No se puede pagar dos veces en la misma ronda y solo pueden pagar los miembros.
- `recipient()`: dice a quién le toca recibir en la ronda actual.
- `is_round_complete()`: dice si todos ya pagaron en la ronda actual.
- `next_round()`: el administrador pasa a la siguiente ronda, pero solo si todos pagaron.

Nota: este contrato solo lleva el registro y no mueve dinero. Una mejora futura
sería usar un token de Stellar para mover los fondos de verdad.

## Cómo correrlo

Necesitas Rust instalado.

    cargo test

## Pruebas

- `ronda_completa_y_cambia_receptor`: dos miembros pagan, la ronda se completa y el receptor cambia al segundo miembro.
- `no_avanza_sin_pagos`: si falta algún pago, `next_round` falla.