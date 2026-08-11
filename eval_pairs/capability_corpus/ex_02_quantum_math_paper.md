# Mathematical Notes on Wavepacket Spreading for Voice Agents

These notes summarize the spoken forms we expect when LaTeX plugins convert physics Markdown into conversational prose. The target audience is quality reviewers who listen to Kokoro renderings side by side with raw Markdown synthesis.

## Hamiltonian Overview

The total energy operator is written $H = T + V$ where the kinetic term expands as $T = \frac{p^2}{2m}$. Reviewers should hear “H equals T plus V” rather than backslash tokens. In three-dimensional Hilbert space the time-independent equation is:

$$
-\frac{\hbar^2}{2m} \nabla^2 \Psi(\mathbf{r}) + V(\mathbf{r})\Psi(\mathbf{r}) = E\Psi(\mathbf{r})
$$

## Normalization Narrative

Probability density $\rho(x)$ must satisfy normalization so that the integral of the squared amplitude equals one. Spoken output must mention the limits from negative infinity to positive infinity without reading raw `\int` glyphs.

$$
\int_{-\infty}^{\infty} |\Psi(x, t)|^2 dx = 1
$$

A free particle plane wave $\Psi(x, t) = A e^{i(kx - \omega t)}$ should become a natural sentence about amplitude, wavevector, and angular frequency. Currency examples such as “It cost $5 to calibrate detectors” must remain untouched by the math speechifier.

## Spreading Law

For a Gaussian packet centered at the origin, variance grows according to:

$$
\sigma(t) = \sigma_0 \sqrt{1 + \left(\frac{\hbar t}{2m \sigma_0^2}\right)^2}
$$

As $t \to \infty$ the spatial spread approaches linear growth. Evaluation judges should confirm no dollar delimiters remain in the voiceover text and that fractions and square roots are verbalized.
