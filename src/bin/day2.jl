using DelimitedFiles

struct Movement 
    direction::String
    mag::Int32

    function Movement(s::String) 
        dir, mag = split(s, " ")
        new(dir, parse(Int32, mag))
    end
end

Base.@kwdef mutable struct Location 
    depth:: Int32 = 0
    horizontal:: Int32 = 0
    aim:: Int32 = 0
end
#p1
function move_by(curr::Location, move::Movement)
    if move.direction == "forward"
        curr.horizontal += move.mag
    elseif  move.direction == "up"
        curr.depth -= move.mag
    elseif move.direction == "down"
        curr.depth += move.mag
    else
        error
    end
end
# p2
function aim_by(curr::Location, move::Movement)
    if move.direction == "forward"
        curr.horizontal += move.mag
        curr.depth += curr.aim * move.mag
    elseif  move.direction == "up"
        curr.aim -= move.mag
    elseif move.direction == "down"
        curr.aim += move.mag
    else
        error
    end
end

function solve_p1(data::Vector{Movement})  
    starting_position = Location()
    data .|> m -> move_by(starting_position, m)
    starting_position.horizontal * starting_position.depth
end
function solve_p2(data::Vector{Movement})  
    starting_position = Location()
    data .|> m -> aim_by(starting_position, m)
    starting_position.horizontal * starting_position.depth
end

function main()
    data::Vector{Movement} = Movement.(readlines("../inputs/day2.txt"))
    @show solve_p1(data)
    @show solve_p2(data)
end

main()

