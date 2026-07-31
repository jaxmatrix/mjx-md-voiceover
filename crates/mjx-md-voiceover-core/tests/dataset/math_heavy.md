# Mathematical Foundations of Quantum Mechanics

This paper outlines the mathematical formulation of wavepacket dynamics.

## Kinetic Energy & Hamiltonian

The total energy operator is given by $H = T + V$, where $T = \frac{p^2}{2m}$.

In three-dimensional Hilbert space, the time-independent Schrödinger equation is expressed as:

$$
-\frac{\hbar^2}{2m} \nabla^2 \Psi(\mathbf{r}) + V(\mathbf{r})\Psi(\mathbf{r}) = E\Psi(\mathbf{r})
$$

## Normalization & Probability Density

The probability density function $\rho(x)$ must satisfy the normalization condition:

$$
\int_{-\infty}^{\infty} |\Psi(x, t)|^2 dx = 1
$$

where $\Psi(x, t) = A e^{i(kx - \omega t)}$ represents a free particle plane wave with wavevector $k$ and angular frequency $\omega$.

## Gaussian Wavepacket Evolution

For a Gaussian wavepacket centered at $x_0 = 0$, the variance expands over time according to:

$$
\sigma(t) = \sigma_0 \sqrt{1 + \left(\frac{\hbar t}{2m \sigma_0^2}\right)^2}
$$

As $t \to \infty$, the spatial spread approaches linear growth.
