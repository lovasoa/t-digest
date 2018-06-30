u
<- rnorm(100000)*100
p <- c(0,5, 1, 5, 25, 50, 75, 95, 99, 99.5)/100
a <- quantile(u,p)
write(u, file="~/src/t-digest/data/large-normal.dat", ncolumns=1)
write(a, file="~/src/t-digest/data/large-normal.sta", ncolumns=1)
