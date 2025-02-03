# Th. Seidel method (sufficient convergence clause)
## Method definition
x_{n+1} = x_{n+1} * B1 + x_{n} * B2 + C 

```
Ax = b <=> Dx + (A1+A2)x = b => Dx{n+1} = -A1x{n+1} - A2x{n} + b <=>
<=> D(x{n+1} - x{n}) + Dx{n} + A1x{n+1} + A2x{n} = b <=>
<=> (D+A1)(x{n+1} - x{n}) + Dx{n} + A1x{n} + A2x{n} = b <=>
<=> (D+A1)(x{n+1} - x{n}) + Ax{n} = b
```

## Statement
(||B2|| / (1 - ||B1||) < 1) => (∀x_{0} Seidel method converges)

## Proof
Seidel method in non-canonical form:
```
x_{n+1} = x_{n+1} * B1 + x_{n} * B2 + C }
                                        }=>
x_{n+1} = x_{n+1} * B1 + x_{n} * B2 + C }

=>  x_{n+1} - x = (x_{n+1} - x) * B1 + (x_{n} - x) * B2 =>
=>||x_{n+1} - x|| <= ||x_{n+1} - x|| * ||B1|| + ||x_{n} - x|| * ||B2|| =>
=>||x_{n+1} - x|| * (1 - ||B1||) <= ||x_{n} - x|| * ||B2|| =>
=>||x_{n+1} - x|| <= ||x_{n} - x|| * ||B2|| / (1 - ||B1||) =>
=> The statement above is true.
```

# Th. Relaxation method (sufficient convergence clause)
## Method definition
```
 x{n+1} = (1 - ω) *  x{n} + ω * B1 * x{n+1} + ω * B2 * x{n} + ωC

Ex{n+1} = (1 - ω) * Ex{n} + ω * B1 * x{n+1} + ω * B2 * x{n} + ωC
Dx{n+1} = (1 - ω) * Dx{n} - ω * A1 + x{n+1} - ω * A2 * x{n} + ωb
(D + ωA1)(x{n+1} - x{n}) + (ωD+ωA1)x{n} + ωA2x{n} = ωb
(D + ωA1)(x{n+1} - x{n}) / ω + Ax{n} = b
```

Canonical:
(D + ω * A1) (x{n} - x{n-1}) / ω + A * x{n-1} = b
