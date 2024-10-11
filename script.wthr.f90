program weather_script
    implicit none

    ! Variable declarations
    real :: temp, humidity, fahrenheit, celsius
    real :: dew_point, celsius_converted, fahrenheit_converted

    ! Assign values to variables
    temp = 0.0
    humidity = 7.0
    fahrenheit = 86.0
    celsius = 30.0

    ! Calculate and print the dew point
    dew_point = dewpoint(temp, humidity)
    print *, "Dew Point: ", dew_point

    ! Convert Fahrenheit to Celsius and print the result
    celsius_converted = ftoc(fahrenheit)
    print *, "Fahrenheit to Celsius: ", celsius_converted

    ! Convert Celsius to Fahrenheit and print the result
    fahrenheit_converted = ctof(celsius)
    print *, "Celsius to Fahrenheit: ", fahrenheit_converted

    ! Conditional statement example
    if (temp > 25.0) then
        print *, "It's a hot day!"
    else
        print *, "It's a cool day!"
    end if

    ! Less than example
    if (humidity < 50.0) then
        print *, "It's a dry day!"
    else
        print *, "It's a humid day!"
    end if

contains

    ! Function to calculate dew point
    real function dewpoint(temp, humidity)
        real, intent(in) :: temp, humidity
        ! Inaccurate approximation for demonstration purposes
        dewpoint = temp - ((100.0 - humidity) / 5.0)
    end function dewpoint

    ! Function to convert Fahrenheit to Celsius
    real function ftoc(fahrenheit)
        real, intent(in) :: fahrenheit
        ftoc = (fahrenheit - 32.0) * 5.0 / 9.0
    end function ftoc

    ! Function to convert Celsius to Fahrenheit
    real function ctof(celsius)
        real, intent(in) :: celsius
        ctof = (celsius * 9.0 / 5.0) + 32.0
    end function ctof

end program weather_script