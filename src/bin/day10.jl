# Ugly imperative mess!

function part1(lines::Vector{String})::Int64
    # Find 1st illegeal character per line
    invalidchars::Vector{Char} = []
    openchar = Set(['(', '[', '{', '<'])
    close2open = Dict(')' => '(', ']' => '[', '}' => '{', '>' => '<')
    for line in lines
        stack::Vector{Char} = []
        for c in collect(line)
            if c ∈ openchar
                push!(stack, c)
            else
                if isempty(stack)
                    push!(invalidchars, c)
                    break
                else
                    lastelement = pop!(stack)
                    if lastelement != close2open[c]
                        push!(invalidchars, c)
                        break
                    end
                end
            end
        end
    end

    illegalcharpoints = Dict(')' => 3, ']' => 57, '}' => 1197, '>' => 25137)
    sum(map(x -> illegalcharpoints[x], invalidchars))
end

function costcompletion(stack::Vector{Char})::Int64
    closecost = Dict('(' => 1, '[' => 2, '{' => 3, '<' => 4)
    reduce((acc, x) -> acc * 5 + x, map(c -> closecost[c], reverse(stack)), init = 0)
end

function part2(lines::Vector{String})::Int64
    stacks::Vector{Vector{Char}} = []
    openchar = Set(['(', '[', '{', '<'])
    close2open = Dict(')' => '(', ']' => '[', '}' => '{', '>' => '<')
    for line in lines
        stack::Vector{Char} = []
        ignoreline = false
        for c in collect(line)
            if c ∈ openchar
                push!(stack, c)
            else
                if isempty(stack)
                    ignoreline = true
                    break
                else
                    lastelement = pop!(stack)
                    if lastelement != close2open[c]
                        ignoreline = true
                        break
                    end
                end
            end
        end
        if !ignoreline
            push!(stacks, stack)
        end
    end
    stackscost = sort(costcompletion.(stacks))
    stackscost[div(length(stackscost), 2)+1]
end

function main()
    data::Vector{String} = collect(eachline("../inputs/day10.txt"))

    @show part1(data)
    @show part2(data)
end

main()