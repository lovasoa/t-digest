# Test and answer data generation with R

## lrg-uniform

Uniform distribution of 100,000 records

```
u <- runif(100000)*100
p <- c(0,5, 1, 5, 25, 50, 75, 95, 99, 99.5)/100
a <- quantile(u,p)
write(u, file="~/src/t-digest/data/lrg-uniform-dataset.dat", ncolumns=1)
write(a, file="~/src/t-digest/data/lrg-uniform-quantiles.dat", ncolumns=1)
```

## small-uniform

Uniform distribution of 50 records

```
u <- runif(50)*100
p <- c(0,5, 1, 5, 25, 50, 75, 95, 99, 99.5)/100
a <- quantile(u,p)
write(u, file="~/src/t-digest/data/small-uniform-dataset.dat", ncolumns=1)
write(a, file="~/src/t-digest/data/small-uniform-quantiles.dat", ncolumns=1)
```

## lrg-skew

Skewed distribution of 100,000 records

```
u <- rbeta(100000,5,1.5)
p <- c(0,5, 1, 5, 25, 50, 75, 95, 99, 99.5)/100
a <- quantile(u,p)
write(u, file="~/src/t-digest/data/lrg-skew-dataset.dat", ncolumns=1)
write(a, file="~/src/t-digest/data/lrg-skew-quantiles.dat", ncolumns=1)
```

## small skew

Skewed distribution of 50 records

```
u <- rbeta(50,5,1.5)
p <- c(0,5, 1, 5, 25, 50, 75, 95, 99, 99.5)/100
a <- quantile(u,p)
write(u, file="~/src/t-digest/data/small-skew-dataset.dat", ncolumns=1)
write(a, file="~/src/t-digest/data/small-skew-quantiles.dat", ncolumns=1)
```

## mass-point1

Data set 1 with a large occurrence of the same value followed by the remaining distribution all to the other side of that value

```
m <- rep(16.482368, 1500)
u <- runif(8500)*80+20
c <- c(m, u)
p <- c(0,5, 1, 5, 25, 50, 75, 95, 99, 99.5)/100
a <- quantile(c,p)
write(c, file="~/src/t-digest/data/mass-point1-dataset.dat", ncolumns=1)
write(a, file="~/src/t-digest/data/mass-point1-quantiles.dat", ncolumns=1)
```

## mass-point2

Data set 2 with a large occurrence of the same value followed by the remaining distributionall to the other side of that value

```
m <- rep(86.85518, 1500)
u <- runif(8500)*80
c <- c(u, m)
p <- c(0,5, 1, 5, 25, 50, 75, 95, 99, 99.5)/100
a <- quantile(c,p)
write(c, file="~/src/t-digest/data/mass-point2-dataset.dat", ncolumns=1)
write(a, file="~/src/t-digest/data/mass-point2-quantiles.dat", ncolumns=1)
```



