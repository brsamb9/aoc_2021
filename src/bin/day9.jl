# Utility functions
function adjacentpoints(p::CartesianIndex)::Vector{CartesianIndex}
    i, j = p[1], p[2]
    CartesianIndex.([(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)])
end

function extractlowpoints(grid::Array{Int64,2})::Vector{Int64}
    lowpoints::Vector{Int64} = []
    rows, cols = size(grid)
    for indx in eachindex(view(grid, 1:rows, 1:cols))
        # Skip max
        currentpoint = grid[indx]
        currentpoint == 9 && continue
        # Else check if lowest
        indices = [p for p in adjacentpoints(indx) if checkbounds(Bool, grid, p)]
        all(x -> currentpoint < x, grid[indices]) && push!(lowpoints, currentpoint)
    end
    lowpoints
end

cost(lowpoints::Vector{Int64}) = sum(lowpoints .+ 1)

function part1(grid::Array{Int64,2})::Int64
    grid |> extractlowpoints |> cost
end

# Pretty much indenticalto above but indices rather than values
function extractlowpointsindices(grid::Array{Int64,2})::Vector{CartesianIndex}
    lowpointsindices::Vector{CartesianIndex} = []
    rows, cols = size(grid)
    for indx in eachindex(view(grid, 1:rows, 1:cols))
        currentpoint = grid[indx]
        currentpoint == 9 && continue
        indices = [p for p in adjacentpoints(indx) if checkbounds(Bool, grid, p)]
        all(x -> currentpoint < x, grid[indices]) && push!(lowpointsindices, indx)
    end
    lowpointsindices
end

function basinsize(grid::Array{Int64,2}, lowpoint::CartesianIndex)::Int64
    indicestocheck::Set{CartesianIndex} = Set([lowpoint])
    basin::Set{CartesianIndex} = Set([])

    while !isempty(indicestocheck)
        next = pop!(indicestocheck)
        push!(basin, next)
        for idx in [p for p in adjacentpoints(next) if checkbounds(Bool, grid, p)]
            grid[idx] != 9 && idx ∉ basin && push!(indicestocheck, idx)
        end
    end

    length(basin)
end

function part2(grid::Array{Int64,2})::Int64
    indices = extractlowpointsindices(grid)
    largestthreebasin = sort(map(x -> basinsize(grid, x), indices))[end-2:end]
    prod(largestthreebasin)
end

function main()
    data::Array{Int64,2} =
        mapreduce(x -> parse.(Int, collect(x)), hcat, eachline("../inputs/day9.txt"))
    @show part1(data)
    @show part2(data)
end

main()