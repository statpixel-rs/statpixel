import nbt from 'prismarine-nbt';
import util from 'node:util';
import fs from 'node:fs';

const raw = 'H4sIAAAAAAAAAO1WS3PaVhS+xHGLaVr3NW0mzUzVaWeaTKJEb4S7woCFCIIYZGy0yVxJV0KgB9UjNmy76qL7TCfretvf4J+SH9LpETZ2HJKJ28x0kSkLIZ3nd8/5zpFKCG2ggldCCF2/hq55doEuoPValIVpoYTWUuxuoOsktEYo/xXQR3uhGRM8waZPCmtoo+nZZMfHbgLav0roQ9tLpj6egVM7ikkRpF+iz0+Oy02C/XS0RZ0c43sszyxkdeKQMCGnQoFBN0HWnxJin5kJ8Fe5c698F30Pmh0cB17oUjtRnGbhmZfEnNrwzF1ItQn3/czFMdXLkhGlIlCWlRiHabKw5uDqvHj+B7XIAroH4HT75FjSPRITm9qOwizZonYITlLqjvBQuIu+A6taFJheSBIqHRHqZVMQRc4iySHBcQ4OsgjU1CMWyTOWIyf3WQTRiB+FVDUOovg+VYsjMFo+4dBenLD/c4YB9kL64BJ0qSy+ePYMfQV3r5Th7BC3QFM1Pd9LZ4AfTEj8Y0KBv0XQtxeRvCDIQrCh0ohKYxxMfQiFNsDAAkRJHok9OYbK+CfHE3xyHFPLp/yqDfWmWqO2u129T12YraF1K/KjGP3y509FdL2DA4K+BrUWJckMiglgQsi3HUVpgkpos3EEqatpGntmlpJkDW3GULx09iSbujG2SU6lQhEVg8j2HKg2Wg/ySMWcneiTnUZPa3T07pMFjBK6kZMTDpcngVilJO//kxj6v+ArAMoy8PtBZE3MV7BMc7Jp04LImLRMWInmeYHheatckStsEW2kXkCSFCqDPhUfsuJDjqdYZkuUqaqG0DX0QR0H2CUov8+H5f4rw/JvBuTj1QGpiFcaEP4KAyK+PCDvBdeFtzK03VAUtaO8lqTj598sSXprlaRt4rqQ8d15+tk5T5dgzpkosBXLlsoszTEORwusw9KmZEq0Y9mm4HAci4XKChMrCyayW2LlDUy89z8T/3MmSm9lYq3Z6OuP21W98TouHvx6Y8nF26tcrI2g/9ChlLwzG784Z+MFoHM+srYjO4Ij0oQhMi1AU2nZJgwtmNjBDGhZYl3i4+aSj9yWKL2Bjr+9QsdV/pX6k8z3u4chiQGkCkBkjPmyJXO0I8tlmmccTGNbJvmISBVTsBxBssDvMbSRxKlHkg1UTMlRmsUkKZ0ef32A/YwUfieHkavWWgzeZ32L743Mg6qn1iNX04ecNrdYbe7yWl090rzDR2qt6lnN1lMj8BNjz5+oXlVSa+pM29+bd8b2pDNX2SE3ZAy94w3HKtfVW+OO0phr9e1RR+n5Q09Nal7VVcPtmckZU1MZdIeQ9zROq2kc+HPjoNXZnS3tOlODE0d2czAzBi3fOhhMrWBwmrfZm9n7e2d2PZ80eyzo5qe6JMean0vvM373siy3H8zMmup2vaqHmz3GqkdP2/xFjHbATs1gMLaCncCuiZlxsPvUVgbCAkdfZrW6yxlKzzP0Cacpw7k23xU0pTPSxqpoKA2+U6/OtPnkaLivMppuT7R5yx8GUCfdglruzjVlMOqMG4eGssd269UjI9ibq260wObswn+TeeTsQp+uunuu+JnI/dPPxPdi93Bv3T3NRltr6C/tneWuubm6a5rED0j6zntm83zPnCa/2DGEYRzJMWmW4+Cdh+0yLcuyTVsswZKDiVAh9sqOkRfvvC1RoB5f2jFrCP0NVd3H67cMAAA=';
const buf = Buffer.from(raw, 'base64');

const { parsed, type } = await nbt.parse(buf);

fs.writeFileSync('./parsed.json', JSON.stringify(parsed, null, '\t'));
