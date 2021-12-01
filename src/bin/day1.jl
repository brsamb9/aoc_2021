using DelimitedFiles

solve_day1_p1(data::Vector{Int64})::Int64 = sum(diff(data) .> 0)
solve_day1_p2(data::Vector{Int64})::Int64 = solve_day1_p1([sum(data[i-2:i]) for i in 3:length(data)])

function main()
    data::Vector{Int64} = vec(open(readdlm, "../inputs/day1.txt"))
    @show solve_day1_p1(data), solve_day1_p2(data)
end

main()