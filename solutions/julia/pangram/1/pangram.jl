"""
    ispangram(input)

Return `true` if `input` contains every alphabetic character (case insensitive).

"""
function ispangram(input)
    letters = Set(c for c in 'A':'A'+25)

    for c in input
        delete!(letters, uppercase(c))
        if length(letters) == 0
            return true
        end
    end

    return false
end
