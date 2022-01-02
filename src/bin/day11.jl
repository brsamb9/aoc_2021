using IterTools

function step!(state::Matrix{Int8})::Int64
    state .+= 1
    hasflashedinstep = zeros(Bool, size(state))

    while true
        toflash = findall(x -> x > 9, state)
        isempty(toflash) && break

        state[toflash] .= 0
        hasflashedinstep[toflash] .= true

        foreach(idx -> increaseneighbours!(state, idx), toflash)
    end
    state[hasflashedinstep] .= 0
    return sum(hasflashedinstep)
end

function increaseneighbours!(state::Matrix{Int8}, idx::CartesianIndex)
    x, y = idx.I
    adjadenctindices = [
        CartesianIndex(x + i, y + j) for (i, j) in IterTools.product(-1:1, -1:1) if
        !(i == j == 0) && checkbounds(Bool, state, x + i, y + j)
    ]
    state[adjadenctindices] .+= 1
end

part1(startstate::Matrix{Int8})::Int64 = reduce(+, map(_ -> step!(startstate), 1:99))
function part2(startstate::Matrix{Int8})::Int64
    i = 1
    while true
        step!(startstate) == 100 && return i
        i += 1
    end
end

function main()
    octopusstate::Matrix{Int8} =
        mapreduce(x -> parse.(Int, collect(x)), hcat, eachline("../inputs/day11.txt"))

    @show part1(deepcopy(octopusstate))
    @show part2(deepcopy(octopusstate))
end
main()

