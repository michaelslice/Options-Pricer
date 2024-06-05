## Options-Pricer
A simple command line tool used to price European Options and American Options using the Black Scholes Equation, and the Binomial Options Pricing Model.

## Black Scholes Model

![image](https://github.com/michaelslice/Options-Pricer/assets/110714088/9f46f503-c36e-450c-9628-6601a94684a4)
![variables](https://github.com/michaelslice/Options-Pricer/assets/110714088/20f2ab8d-ec45-4285-896f-3392d5e9d9b9)

## Binomial Options Pricing Model

The stock tree is represented using node $(i, j)$ with initial stock price $S0$ <br>

$S{_i}{_j} = S_0u^jd^{i-j}$ <br>

Where $C_{i}{_j}$ represents the contract price at each node $(i, j)$, and $C{_N}{_j}$ represents the final payoff function that we can define. <br>

In this project a European call option is priced using the equation, $C{_N}{_j} = max(S{_N}{_j} - K, 0)$


