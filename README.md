# Optimization Model

## First Principle

### Goal
- Minimization of an energy function to achieve an optimal or stable state.

### Method
1. Define an energy function that quantifies the "cost" or "stability" of a system’s state.
2. Find the configuration of variables that minimizes this energy function.

### Mechanism

**Generalization:**

$$
E = \pm \alpha \sum_{ij} A_{ij} b_i b_j \pm \beta \sum_i C_i b_i
$$

where:
- $E, \alpha, \beta \in \mathbb{R}$
- $A \in \mathbb{R}^{N \times N}$ (where $N$ is the number of variables)
- $b = [b_1, b_2, \ldots, b_N]^T \in \mathbb{R}^N$
- $C = [C_1, C_2, \ldots, C_N]^T \in \mathbb{R}^N$

**Explain:**
- Pairwise interaction term ($A_{ij} b_i b_j$): Models pairwise relationships or couplings between variables. Coefficient $\alpha$ adjusts the strength of interactions.
- Linear term ($C_i b_i$): Encodes external influences or biases on individual variables. Coefficient $\beta$ scales their effect.
- The minimization process reflects a trade-off between these terms, tailored to the specific context of each model, with the $\pm$ signs allowing positive or negative contributions.

## Examples

### 1. [Markowitz Model](https://en.wikipedia.org/wiki/Markowitz_model#:~:text=In%20finance%2C%20the%20Markowitz%20model,portfolios%20of%20the%20given%20securities.) - Portfolio Optimization

**Equation:**

$$
F = \frac{1}{2} \sum_{ij} C_{ij} n_i n_j - \zeta \sum_i R_i n_i
$$

where:
- $F$: Objective function (risk minus return).
- $C_{ij}$: Covariance between assets $i$ and $j$, representing the risk contribution.
- $n_i$: Weight of asset $i$ (proportion of portfolio invested).
- $\zeta$: Risk tolerance parameter.
- $R_i$: Expected return of asset $i$.

**Principle:**
Minimizing $F$ seeks an optimal portfolio allocation that balances risk and return, a cornerstone of Modern Portfolio Theory.

### 2. [Sherrington-Kirkpatrick Model](https://en.wikipedia.org/wiki/Spin_glass#Sherrington%E2%80%93Kirkpatrick_model) - Spin Glasses

**Equation:**

$$
H = -\frac{1}{\sqrt{N}} \sum_{i < j} J_{ij} S_i S_j - \sum_i h_i S_i
$$

where:
- $H$: Hamiltonian - the total energy of the spin glass system.
- $J_{ij}$: Random interaction strength between spins $i$ and $j$ (e.g., Gaussian distributed with mean 0 and variance 1).
- $S_i \in \lbrace-1, +1\rbrace$: Spin variable at site $i$.
- $h_i$: External field at site $i$ (can be uniform $h$ in some formulations).

**Principle:** Minimizing $H$ finds the ground state of the disordered system. This model explains complex behaviors in disordered materials, including frustration and rugged energy landscapes.

### 3. [Ising Model](https://en.wikipedia.org/wiki/Ising_model) - Magnetic Systems

**Equation:**

$$
H(\sigma) = - \sum_{\langle ij \rangle} J_{ij} \sigma_i \sigma_j - \mu \sum_i h_i \sigma_i
$$

where:
- $H(\sigma)$: Hamiltonian for spin configuration $\sigma = (\sigma_1, \sigma_2, \ldots, \sigma_N)$.
- $J_{ij}$: Coupling between neighboring spins $i$ and $j$.
- $\sigma_i \in \lbrace-1, +1\rbrace$: Spin at site $i$.
- $\mu$: Magnetic moment of a single spin (often set to 1 or absorbed into $h_i$).
- $h_i$: External magnetic field at site $i$.

**Principle:** Minimizing $H$ finds the configuration with lowest energy, representing magnetic ordering under short-range interactions. The Ising model is fundamental for understanding phase transitions.

### 4. [Hopfield Model](https://en.wikipedia.org/wiki/Hopfield_network) - Neural Network

**Equation:**

$$
E = -\frac{1}{2} \sum_{ij} w_{ij} v_i v_j - \sum_i \theta_i v_i
$$

where:
- $E$: Energy function, represents the network's stability.
- $w_{ij}$: Weights of the learning rule between neuron $i$ and $j$ (symmetric, $w_{ij} = w_{ji}$).
- $v_i$: State of neuron $i$ (e.g., 0 or 1, or $\pm 1$ in some formulations).
- $\theta_i$: Activation threshold of neuron $i$.

**Principle:**
Minimizing $E$ guides the network toward stable attractor states, enabling it to recall patterns from partial or noisy inputs. It’s an early example of energy-based learning.

### 5. [Graph Cuts for Image Segmentation](https://en.wikipedia.org/wiki/Graph_cuts_in_computer_vision) - Computer Vision

**Equation:**

$$
E(L) = \sum_{(p,q) \in N} V_{pq}(L_p, L_q) + \sum_{p} D_p(L_p)
$$

where:
- $E(L)$: Total energy for a labeling $L = \{L_p\}$.
- $L_p$: Label assigned to pixel $p$ (e.g. foreground or background).
- $D_p(L_p)$: Data term — cost of assigning label $L_p$ to pixel $p$, encourages data fidelity.
- $V_{pq}(L_p, L_q)$: Smoothness term — cost of label discontinuity between neighboring pixels $p$ and $q$, encourages label coherence.
- $N$: Set of adjacent pixel pairs (i.e. edges in the image grid graph).

**Principle:**
Minimizing $E(L)$ balances fidelity to observed data (e.g. pixel intensities) with spatial smoothness (encouraging neighboring pixels to share labels).
