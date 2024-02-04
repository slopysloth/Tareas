function get_zodiac(dia, mes) {
    switch (mes) {
        case 1:
            if (dia <= 19) return "Capricornio";
            else return "Acuario";

        case 2: // febrero
            if (dia <= 18) return "Acuario";
            else return "Piscis";

        case 3: // marzo
            if (dia <= 20) return "Piscis";
            else return "Aries";

        case 4: // abril
            if (dia <= 20) return "Aries";
            else return "Tauro";

        case 5: // mayo
            if (dia <= 20) return "Tauro";
            else return "Géminis";

        case 6: // junio
            if (dia <= 24) return "Géminis";
            else return "Cáncer";

        case 7: // julio
            if (dia <= 22) return "Cáncer";
            else return "Leo";

        case 8: // agosto
            if (dia <= 23) return "Leo";
            else return "Virgo";

        case 9: // septiembre
            if (dia <= 23) return "Virgo";
            else return "Libra";

        case 10: // octubre
            if (dia <= 22) return "Libra";
            else return "Scorpio";

        case 11: // nov
            if (dia <= 22) return "Scorpio";
            else return "Sagitario";

        case 12: // dic
            if (dia <= 22) return "Capricornio";
    }
}

const dia = parseInt(prompt("Dia de nacimiento"));
const mes = parseInt(prompt("Mes de nacimiento"));

if (!dia && !mes) alert("Respuestas invalidas, dia y mes deben ser numeros");

alert(get_zodiac(dia, mes));
