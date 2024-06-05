## Options-Pricer
A simple command line tool used to price European Options using the Black-Scholes Equation, and the Binomial Options Pricing Model.

## Black-Scholes Model

### Black-Scholes Inputs

- $S$ = underlying price ($$$ per share)
- $K$ = strike price ($$$ per share)
- $σ$ = volatility (% p.a.)
- $r$ = continuously compounded risk-free interest rate (% p.a.)
- $q$ = continuously compounded dividend yield (% p.a.)
- $t$ = time to expiration (% of year)
  
### Call and Put Option Formulas
Call option $(C)$ and put option $(P)$ prices are calculated using the following formulas: <br>

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/8f8c6b2c-a4ed-4ac0-a64a-b2961b6fc0a5)

### Standard Normal Cumulative Distribution Function N(x)

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/aeff0353-46c9-416c-ab93-470d45d81bb6)

### D1 & D2 Formulas

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/bafa79f2-10fb-44b5-ae53-39335644296b)

## Formulas for Greeks 

###  Standard Normal Probability Density Function

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/a4052476-1393-48f6-8d62-c818135a710c)

### Delta (Δ)
Delta is the first derivative of option price with respect to underlying price $S$. The formulas for call and put option delta are the following: <br>

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/415a282e-c156-4b4e-9470-6559c585d76e)

### Gamma (Γ)
Gamma is the second derivative of option price with respect to underlying price $S$. It is the same for calls and puts. <br>

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/ed3dec67-6567-4b36-9a5d-a97b292c568d)

### Theta (Θ)
Theta is the first derivative of option price with respect to time to expiration $t$. <br>

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/944b9215-ea72-44eb-b758-c5cb59edf5be)

$T$ is the number of days per year. <br>
If $T$ is **calendar days** (365), then the resulting theta is change in option price per one calendar day (or 1/365 of a year). <br>

### Vega (ν)
Vega is the first derivative of option price with respect to volatility $σ$. It is the same for calls and puts. <br>

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/088bdac9-358e-4845-84cb-57f0075ec403)

### Rho (ρ)
Rho is the first derivative of option price with respect to interest rate $r$. It is different for calls and puts. <br>

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/e7085795-67ea-42d8-aeea-cc6c96326482)

## Binomial Options Pricing Model

The binomial tree is represented as a network of nodes as $(i, j)$, with i representing the time steps and j representing the number of ordered price outcomes
(lowest - or bottom of tree - to highest) <br>

### Binomial Options Inputs

- $S$ = underlying price ($$$ per share)
- $K$ = strike price ($$$ per share)
- $σ$ = volatility (% p.a.)
- $r$ = continuously compounded risk-free interest rate (% p.a.)
- $q$ = continuously compounded dividend yield (% p.a.)
- $t$ = time to expiration (% of year)
- $n$ = number of steps in the tree
- $u$ = up factor ($$$ increase at each step)
- $d$ = down factor ($$$ decrease at each step)     

## Binomial Tree Representation

The stock tree is represented using nodes $(i, j)$, with initial stock price $S0$ <br>

$S{_i}{_j} = S_0u^jd^{i-j}$ <br>

Where $C_{i}{_j}$ represents the contract price at each node $(i, j)$, and $C{_N}{_j}$ represents the final payoff function that we can define. <br>

In this project a European call option is priced using the equation, $C{_N}{_j} = max(S{_N}{_j} - K, 0)$

## How to Use

1. `git clone https://github.com/michaelslice/Options-Pricer.git`
2. `cd Options-Pricer`
3. `cargo run`
