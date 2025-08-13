"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
    counter = Dict()
    counter['A'] = 0
    counter['C'] = 0
    counter['G'] = 0
    counter['T'] = 0

    for gene in strand
        if gene == 'A'
            counter['A'] += 1
        elseif gene == 'C'
            counter['C'] += 1
        elseif gene == 'G'
            counter['G'] += 1
        elseif gene == 'T'
            counter['T'] += 1
        else
            throw(DomainError(gene, "expected: A|C|G|T"))
        end
    end

    counter
end

