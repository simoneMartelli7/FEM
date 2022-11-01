using Plots

function main()
    f(x) = cos(x) 

    plot(0:0.1:4*π, f)
    
    Plots.gui()
end

main()