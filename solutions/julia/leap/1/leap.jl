"""
    is_leap_year(year)

Return `true` if `year` is a leap year in the gregorian calendar.

"""
function is_leap_year(year)
    is_leap_year = false
    
    if year % 4 == 0
        is_leap_year = true
    end

    if year % 100 == 0
        is_leap_year = false
    end
       
    if year % 400 == 0
        is_leap_year = true
    end

    is_leap_year
end

