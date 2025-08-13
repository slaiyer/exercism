"""
    is_leap_year(year)

Return `true` if `year` is a leap year in the gregorian calendar.

"""
function is_leap_year(year)
    div_by_year(year, n) = year % n == 0
    div_by_year(year, 4) && !div_by_year(year, 100) || div_by_year(year, 400)
end
