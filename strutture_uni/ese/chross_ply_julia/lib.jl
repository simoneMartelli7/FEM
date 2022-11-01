using Plots#; pgfplots()

#defines the constan 
π = pi

### functions needed to initialize a stratum
function lambda(θ)
    c = cos(θ)
    s = sin(θ)

    lam = [ c^2 s^2 -2c*s
            s^2 c^2 2c*s
            c*s -c*s c^2-s^2 ]
    return lam
end

function q_p(E₁, E₂, ν₁, G)
    ν₂ = ν₁*E₂/E₁ #second poisson modulus

    #temporary matrix
    q = [ E₁ ν₁*E₂ 0
          ν₁*E₂ E₂ 0
          0 0 (1 - ν₁*ν₂)G ]
    return q/(1 - ν₁*ν₂)
end

function q(E₁, E₂, ν₁, G, θ)  
    qp = q_p(E₁, E₂, ν₁, G)
    l = inv(lambda(θ))
    return l*qp*l'
end


mutable struct Stratum
    E₁
    E₂
    ν₁
    ν₂
    G
    θ
    Q
    z⁺
    z⁻
end

function new_stratum_typed(e1, e2, nu1, g, θ, zt, zb)
    temp = Stratum( e1, e2, nu1, nu1*e2/e1, g, θ, zeros(3), zt, zb)
    q_ = q(e1, e2, nu1, g, θ)
    temp.Q = q_
    return temp
end

#### A, B, D matrices for a single stratum
function a_mat(Stratum)
    return Stratum.Q*(Stratum.z⁺ - Stratum.z⁻)
end

function b_mat(Stratum)
    return (Stratum.z⁺^2 - Stratum.z⁻^2)*Stratum.Q
end

function d_mat(Stratum)
    return (Stratum.z⁺^3 - Stratum.z⁻^3)*Stratum.Q
end

##########################################################
##########################################################
##########################################################

mutable struct Plate4 
    ### this is a collection of 4 strata
    s1
    s2
    s3
    s4
    ### A B D are the rigidity matrices 
    A
    B
    D
    ### a and b are the plate dimension in the (x, y) plane 
    a 
    b
end

### computes A, B, D matrices for the plate
function A_mat(Plate4)
    Plate4.A = a_mat(Plate4.s1) + a_mat(Plate4.s2) + a_mat(Plate4.s3) + a_mat(Plate4.s4)
end

function B_mat(Plate4)
    Plate4.B = 0.5(b_mat(Plate4.s1) + b_mat(Plate4.s2) + b_mat(Plate4.s3) + b_mat(Plate4.s4))
end

function D_mat(Plate4)
    Plate4.D = (d_mat(Plate4.s1) + d_mat(Plate4.s2) + d_mat(Plate4.s3) + d_mat(Plate4.s4))/3
end


### needed to slim down next function
function omog_str(stratum)
    temp = Stratum(stratum.E₁, stratum.E₂, stratum.ν₁, stratum.ν₂, stratum.G, 0, zeros(3), 0, 0)
    println("Insert lamination angle: ")
    temp.θ = parse(Float64, readline())
    println("Insert z bottom coordinate: ")
    temp.z⁻ = parse(Float64, readline())
    println("Insert z top coordinate: ")
    temp.z⁺ = parse(Float64, readline())
    temp.Q = q(temp.E₁, temp.E₂, temp.ν₁, temp.G, temp.θ)
    return temp
end

#initializes a plate in which every stratum is of the same material 
#different lamination angles and thicknesses are allowed 
function new_omogeneous_P4(stratum)
    s1 = stratum
    s2 = omog_str(stratum)
    s3 = omog_str(stratum)
    s4 = omog_str(stratum)
    temp = Plate4( stratum, s2, s3, s4, zeros(3), zeros(3), zeros(3), 0, 0)
    temp.A = A_mat(temp)
    temp.B = B_mat(temp)
    temp.D = D_mat(temp)
    println("Insert dimension in the x-direction: ")
    temp.a = parse(Float64, readline())
    println("Insert dimension in the y-direction: ")
    temp.b = parse(Float64, readline())
    return temp
end

function new_P4_typed(s1, s2, s3, s4)
    temp = Plate4(s1, s2, s3, s4, zeros(3), zeros(3), zeros(3), 0, 0)
    temp.A = A_mat(temp)
    temp.B = B_mat(temp)
    temp.D = D_mat(temp)
    println("Insert dimension in the x-direction: ")
    temp.a = parse(Float64, readline())
    println("Insert dimension in the y-direction: ")
    temp.b = parse(Float64, readline())
    return temp
end


##############################################################################
mutable struct load
    type
    q₀
    x
    y
end

loads = ["1. Uniform", "2. Bisinusoidal", "3. On a Point"]
function new_load()
    println("What kind of load is applied?")
    for i in loads
        println(i)
    end

    choice = parse(Int8, readline())
    if choice != 1 && choice != 2 && choice != 3 
        println("Invalid option")
        exit()
    end

    println("What is the value of q₀?")
    q₀ = parse(Float64, readline())

    if choice == 3
        println("In what point is the load applied?")
        println("X: ")
        x = parse(Float64, readline())
        println("Y: ")
        y = parse(Float64, readline())
        return load(choice, q₀, x, y)
    else 
        return load(choice, q₀, 0, 0)
    end
end


### functions needed for the next one 
function q_uniform(load, precision)
    if load.type != 1
        println("Error: expected uniform load")
        return -1
    end
    q = zeros(precision, precision)
    for i in 1:precision
        for j in 1:precision
            if i%2 == 0 || j%2 == 0
                q[i, j] == 0
            else
                q[i, j] == 16load.q₀/(π^2*i*j)
            end
        end
    end
    return q
end

function q_bisinusoidal(load, precision)
    if load.type != 2
        println("Error: expected Bisinusoidal load")
        return -1
    end

    q = zeros(precision, precision)
    for i in 1:precision
        for j in 1:precision
            q[i, j] = load.q₀
        end
    end

    return q
end

function q_point(load, a, b, precision)
    q = zeros(precision, precision)

    for i in 1:precision
        for j in 1:precision
            q[i, j] = (4load.q₀/(a*b))(sin(j*π*load.x/a))(sin(i*π*load.y/b))
        end
    end

    return q
end


#### solves for the normal deflection (in the z direction)
### takes as arguments a Plate4 and load structs, precision must be an integer 
### and determines the number of elements in the summation
### returns the w, x and y vectors 

function w₀(Plate4, load, precision)
    if load.type == 1
        q = q_uniform(load, precision)
    elseif load.type == 2
        q = q_bisinusoidal(load, precision)
    else
        q = q_point(load, Plate4.a, Plate4.b, precision)
    end

    wmn = zeros(precision, precision)

    wmn[i, j] = q[i, j]/(Plate4.D[1, 1]*(j*π/Plate4.a)^4 + 2(Plate4.D[1, 2] + 2*Plate4.D[3, 3])*(i*π/Plate4.b)^2*(j*π/Plate4.a)^2 + Plate4.D[2, 2]*(i*π/Plate4.b)^4)
        
    w = zeros(precision, precision)
    x = collect(0:Plate4.a/precision:Plate4.a)
    y = collect(0:Plate4.b/precision:Plate4.b)
    for i in 1:precision
        for j in 1:precision
            w[i, j] = wmn[i, j]*sin(j*π*x[j]/Plate4.a)*sin(i*π*y[i]/Plate4.b)
        end
    end

    println("The maximum deflection is ", maximum(w))
    return w
end

function plot_deflection(Plate4, load, precision)
    w = w₀(Plate4, load, precision)
    heatmap(w, show = true)
end



 #### TODO!
 #### the plotting function is currently not working 
 #### additionaly the correctness of the w₀ function needs to be verified 
 #### almost done; fucking finally!