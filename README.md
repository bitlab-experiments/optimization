# Optimization Toolkit

## Overview
This repository provides a toolkit for modeling and solving optimization problems, with a focus on energy-based models. It includes:
- A Rust/Node.js library for defining and evaluating quadratic energy functions.
- Theoretical documentation and examples of optimization models from physics, finance, and machine learning.

## Repository Structure
- `library/`: Rust + Neon-based Node.js library for optimization models.
- `theory/`: Mathematical background and model documentation (`model.md`).
- `simulation/`: (Currently empty) Intended for simulation scripts or notebooks.

## Key Concepts
The core model minimizes an energy function of the form:

$$
E = \alpha \sum_{ij} A_{ij} b_i b_j + \beta \sum_i C_i b_i
$$

This generalizes many problems, including portfolio optimization, spin glasses, neural networks, and image segmentation. See `theory/model.md` for details and examples.

## Library Usage
The `library/` folder contains a Rust library with Neon bindings for Node.js. Main features:
- Define a model with parameters (alpha, beta, A, b, C)
- Compute the energy (score) for a given configuration

### Build Instructions
Requirements: [Node.js and Rust (platform support)](https://github.com/neon-bindings/neon#platform-support)

```sh
cd library
npm install
npm run build
```

### Example (Node.js)
```js
const { createModel } = require('./index.node');
const alpha = 1.0, beta = 0.5;
const a = [[1, 0], [0, 1]];
const b = [1, -1];
const c = [0.2, 0.3];
const model = createModel(alpha, beta, a, b, c);
console.log(model.score); // Energy value
```

### Available Scripts
- `npm install`: Installs dependencies and builds the addon
- `npm run build`: Release build
- `npm run debug`: Debug build
- `npm run cross`: Cross-compile for other platforms
- `npm test`: Run Rust unit tests

## Theory
See [`theory/model.md`](theory/model.md) for:
- Mathematical formulation
- Convex vs. non-convex cases
- Example models: Markowitz, Ising, Hopfield, Graph Cuts, etc.

## License
ISC