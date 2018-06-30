u <- runif(100000)*100
hist(u)
p <- c(0.5, 1, 5, 10, 25, 50, 75, 90, 95, 99, 99.5)/100
sta <- c("PCTL0.5","PCTL1","PCTL5","PCTL10","PCTL25","PCTL50",
         "PCTL75","PCTL90","PCTL95","PCTL99","PCTL99.5",
         "mean","stdev","count","sum",
         quantile(u,p),
         mean(u),sd(u),length(u),sum(u))
write(u, file="~/src/t-digest/data/large-uniform.dat", ncolumns=1)
write(sta, file="~/src/t-digest/data/large-uniform.sta", ncolumns=15)
