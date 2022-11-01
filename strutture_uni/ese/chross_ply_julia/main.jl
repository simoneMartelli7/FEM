include("lib.jl")

function main()
    e1 = 1.11*10^11
    e2 = 8*10^9
    g = 3*10^9
    nu1 = 0.33
    theta = 0
    zb = -0.001
    zt = -0.0005

    s1 = new_stratum_typed(e1, e2, nu1, g, theta, zb, zt)
    s2 = new_stratum_typed(e1, e2, nu1, g, 1.5708, -0.0005, 0.0)
    s3 = new_stratum_typed(e1, e2, nu1, g, 1.5708, 0.0, 0.0005)
    s4 = new_stratum_typed(e1, e2, nu1, g, 0.0, 0.0005, 0.0)


    plate4 = new_P4_typed(s1, s2, s3, s4)
    

    #println(plate4.B)

    load = new_load()
    
    #println(load.q₀)

    println("Insert refinement (must be an integer, better precision for higher values): ")
    precision = parse(Int64, readline())

    w₀(plate4, load, precision)
    

    

end

main()