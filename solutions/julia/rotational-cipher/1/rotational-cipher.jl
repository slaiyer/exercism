function rotate(amount, input)
    out_char = false
    if isa(input, Char)
        out_char = true
        input = string(input)
    end

    output = ""
    for c in input
        C = uppercase(c)
        if !('A' <= C && C <= 'Z')
            output *= c
        else
            base = 'A'
            if islowercase(c)
                base = 'a'
            end
            output *= (((c - base) + amount) % 26) + base
        end
    end

    if out_char
        return Char(output[1])
    end
    output
end
