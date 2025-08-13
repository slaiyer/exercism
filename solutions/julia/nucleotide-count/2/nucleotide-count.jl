"""
    count_nucleotides(strand)

The count of each nucleotide within `strand` as a dictionary.

Invalid strands raise a `DomainError`.

"""
function count_nucleotides(strand)
    nucleotides = "ACGT"
    counter = Dict((n, 0) for n in nucleotides)

    for n in strand
        n ∈ nucleotides || throw(DomainError(n, "valid nucleotides: $(join(nucleotides, ", "))"))
        counter[n] += 1
    end

    counter
end
