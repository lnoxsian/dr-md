# Obsidian Math Test

This document tests KaTeX math rendering in Obsidian.

---

# Inline Math

The quadratic equation is $ax^2 + bx + c = 0$.

Euler's identity:

$e^{i\pi} + 1 = 0$

Pythagorean theorem:

$a^2+b^2=c^2$

Fraction:

$\frac{a}{b}$

Roots:

$\sqrt{2}$

$\sqrt[n]{x}$

Greek:

$\alpha,\beta,\gamma,\delta,\epsilon,\theta,\lambda,\mu,\pi,\sigma,\omega$

Infinity:

$\infty$
  
Summation:

$\sum_{i=1}^{n} i$

Product:

$\prod_{i=1}^{n} i$

Integral:

$\int_0^1 x^2 dx$

Limit:

$\lim_{x\to0}\frac{\sin x}{x}=1$

---

# Display Math

$$
E = mc^2
$$

---

$$
f(x)=x^2+2x+1
$$

---

$$
\frac{d}{dx}(x^n)=nx^{n-1}
$$

---

# Algebra

## Quadratic Formula

$$
x=\frac{-b\pm\sqrt{b^2-4ac}}{2a}
$$

---

## Binomial Theorem

$$
(a+b)^n=\sum_{k=0}^{n}\binom{n}{k}a^{n-k}b^k
$$

---

## Polynomial

$$
P(x)=x^5-3x^3+7x-9
$$

---

# Calculus

## Derivative

$$
\frac{d}{dx}\sin(x)=\cos(x)
$$

---

## Second Derivative

$$
\frac{d^2}{dx^2}x^3=6x
$$

---

## Indefinite Integral

$$
\int x^2dx=\frac{x^3}{3}+C
$$

---

## Definite Integral

$$
\int_0^\pi\sin(x)\,dx=2
$$

---

## Double Integral

$$
\iint_Df(x,y)\,dA
$$

---

## Triple Integral

$$
\iiint_Vf(x,y,z)\,dV
$$

---

## Contour Integral

$$
\oint_Cf(z)\,dz
$$

---

## Partial Derivative

$$
\frac{\partial f}{\partial x}
$$

---

## Gradient

$$
\nabla f
$$

---

## Divergence

$$
\nabla\cdot\vec F
$$

---

## Curl

$$
\nabla\times\vec F
$$

---

## Laplacian

$$
\nabla^2f
$$

---

# Linear Algebra

## Matrix

$$
\begin{bmatrix}
1&2\\
3&4
\end{bmatrix}
$$

---

## Parenthesis Matrix

$$
\begin{pmatrix}
a&b\\
c&d
\end{pmatrix}
$$

---

## Determinant

$$
\begin{vmatrix}
1&2\\
3&4
\end{vmatrix}
=-2
$$

---

## Identity Matrix

$$
I=
\begin{bmatrix}
1&0&0\\
0&1&0\\
0&0&1
\end{bmatrix}
$$

---

## Vector

$$
\vec v=
\begin{bmatrix}
1\\
2\\
3
\end{bmatrix}
$$

---

## Dot Product

$$
\vec a\cdot\vec b
$$

---

## Cross Product

$$
\vec a\times\vec b
$$

---

## Norm

$$
\|\vec x\|
$$

---

# Systems of Equations

$$
\begin{cases}
x+y=2\\
x-y=4
\end{cases}
$$

---

# Alignment

$$
\begin{aligned}
a+b&=c\\
2a+b&=d\\
3a+4b&=e
\end{aligned}
$$

---

# Piecewise

$$
f(x)=
\begin{cases}
x^2,&x>0\\
0,&x=0\\
-x,&x<0
\end{cases}
$$

---

# Probability

Expectation

$$
E[X]=\sum_ix_ip_i
$$

Variance

$$
Var(X)=E[X^2]-E[X]^2
$$

Normal Distribution

$$
f(x)=
\frac{1}{\sqrt{2\pi\sigma^2}}
e^{-\frac{(x-\mu)^2}{2\sigma^2}}
$$

Binomial

$$
P(X=k)=
\binom{n}{k}
p^k(1-p)^{n-k}
$$

---

# Number Theory

Congruence

$$
a\equiv b\pmod n
$$

Greatest Common Divisor

$$
\gcd(a,b)
$$

Least Common Multiple

$$
\operatorname{lcm}(a,b)
$$

---

# Logic

$$
P\land Q
$$

$$
P\lor Q
$$

$$
P\Rightarrow Q
$$

$$
P\Leftrightarrow Q
$$

$$
\forall x\in\mathbb R
$$

$$
\exists x
$$

---

# Set Theory

$$
A\subseteq B
$$

$$
A\cup B
$$

$$
A\cap B
$$

$$
A\setminus B
$$

$$
\emptyset
$$

$$
\mathbb N,\mathbb Z,\mathbb Q,\mathbb R,\mathbb C
$$

---

# Physics

Wave Equation

$$
\psi(x,t)=Ae^{i(kx-\omega t)}
$$

Maxwell

$$
\nabla\cdot\vec E=\frac{\rho}{\epsilon_0}
$$

Schrödinger

$$
i\hbar\frac{\partial\Psi}{\partial t}
=
\hat H\Psi
$$

---

# Chemistry

Reaction

$$
2H_2+O_2\rightarrow2H_2O
$$

Equilibrium

$$
K=\frac{[C]^c[D]^d}{[A]^a[B]^b}
$$

---

# Large Operators

$$
\sum_{n=1}^{\infty}\frac1{n^2}
=
\frac{\pi^2}{6}
$$

---

$$
\prod_{i=1}^{10}i
$$

---

# Accents

$$
\hat x
$$

$$
\bar x
$$

$$
\tilde x
$$

$$
\overline{AB}
$$

$$
\vec F
$$

---

# Fonts

$$
\mathbb R
$$

$$
\mathcal F
$$

$$
\mathfrak g
$$

$$
\mathrm d
$$

$$
\mathbf A
$$

---

# Brackets

$$
\left(
\frac{a+b}{c+d}
\right)
$$

---

$$
\left[
\sum_i x_i
\right]
$$

---

$$
\left\{
\begin{matrix}
a\\
b
\end{matrix}
\right.
$$

---

# Colors (if supported)

$$
\color{red}{x+y}
$$

$$
\color{blue}{E=mc^2}
$$

---

# Spacing

$$
a\,b\quad c\qquad d
$$

---

# Text

$$
\text{Hello Obsidian}
$$

---

# Boxed Equation

$$
\boxed{E=mc^2}
$$

---

# Arrays

$$
\begin{array}{ccc}
1&2&3\\
4&5&6\\
7&8&9
\end{array}
$$

---

# Multi-line Equation

$$
\begin{aligned}
f(x)
&=x^2+2x+1\\
&=(x+1)^2
\end{aligned}
$$

---

# Limits

$$
\lim_{n\to\infty}
\left(1+\frac1n\right)^n=e
$$

---

# Infinite Series

$$
\sum_{n=0}^{\infty}x^n
=
\frac1{1-x}
$$

---

# Fourier Transform

$$
F(\omega)=
\int_{-\infty}^{\infty}
f(t)e^{-i\omega t}dt
$$

---

# Gaussian Integral

$$
\int_{-\infty}^{\infty}
e^{-x^2}dx
=
\sqrt{\pi}
$$

---

# Complex Numbers

$$
z=a+bi
$$

$$
|z|=\sqrt{a^2+b^2}
$$

$$
\arg(z)
$$

---

# Nested Fractions

$$
\frac{1}{
1+\frac{1}{
2+\frac{1}{3}
}}
$$

---

# Huge Expression

$$
\sqrt{
\frac{
\sum_{i=1}^{n}(x_i-\mu)^2
}{
n-1
}
}
$$

---

# End of Test

If every equation above renders correctly, your Markdown math renderer has excellent KaTeX compatibility.    