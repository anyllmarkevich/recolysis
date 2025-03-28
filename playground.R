# Prepare Rust code for use (press cmd+shift+B in R Studio to compile first)
rextendr::document()
# Load Libraries
library(rextendr)
library(recolysis)

### DETERMINISTIC SIMULATION###

# Prepare data
survival_matrix <- matrix(c(0, 0, 0.1, 0.1, 0.8, 0, 0, 0.8, 0.99), ncol = 3, nrow=3, byrow = TRUE)
population_vector <- c(50, 100, 100)
generations <- 50

# Run simulation
simulation_output <- determ_pva(survival_matrix, population_vector, generations)

# Define graph x and y axes
y <- rowMeans(simulation_output)
x <- seq(0, length(y)-1, 1)

# Graph simulation
plot(x, y, type="l", xlab = "Generation", ylab = "Individuals")

###STOCHASTIC SIMULATION###

#Prepare data
survival_matrices <- array(c(0, 0.05, 0.0, 0.0, 0.5, 0.1, 2.0, 0.0, 0.9,
                 0, 0.2, 0.0, 0.0, 0.6, 0.2, 0.5, 0.0, 0.85,
                 0, 0.15, 0.0, 0.0, 0.4, 0.5, 2.2, 0.0, 0.8,
                 0, 0.075, 0.0, 0.0, 0.7, 0.2, 0.7, 0.0, 0.99), dim=c(3, 3, 4))
population_vector <- c(50, 100, 100)
generations <- 50
simulation_iterations <- 10

# Run simulation
simulation_output <- stoch_pva(survival_matrices, population_vector, generations, simulation_iterations)

# Define graph x and y axes
y <- d[, 3, 2] # Select ONE of the simulation iterations, and ONE of the lifestages to graph. Data is currently formatted as [iteration, life stage, generation]
x <- seq(0, length(y)-1, 1)

# Graph simulation
plot(x, y, type="l", xlab = "Generation", ylab = "Individuals")
