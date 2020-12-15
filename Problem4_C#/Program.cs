using System;

namespace Problem4_C_
{
    class Program
    {
        static void Main(string[] args)
        {
            int validCounter = 0;
            string[] blocks = System.IO.File.ReadAllText("input.txt").Split("\n\n");
            foreach (string block in blocks) {
                bool containsByr = false;
                bool containsIyr = false;
                bool containsEyr = false;
                bool containsHgt = false;
                bool containsHcl = false;
                bool containsEcl = false;
                bool containsPid = false;
                string[] lines = block.Split("\n");
                foreach (string line in lines) {
                    string[] tokens = line.Split(" ");
                    foreach(string token in tokens) {
                        string[] parsedToken = token.Split(":");
                        if (parsedToken.Length == 2) {
                            string predicate = parsedToken[0];
                            string suffix = parsedToken[1];
                            int numericSuffix;
                            bool successfullyParsed = Int32.TryParse(suffix, out numericSuffix);
                            switch (predicate) {
                                case "byr": {
                                    if (numericSuffix >= 1920 && numericSuffix <= 2002) {
                                        containsByr = true;
                                    }
                                    break;
                                }
                                case "iyr": {
                                    if (numericSuffix >= 2010 && numericSuffix <= 2020) {
                                        containsIyr = true;
                                    }
                                    break;
                                }
                                case "eyr": {
                                    if (numericSuffix >= 2020 && numericSuffix <= 2030) {
                                    containsEyr = true;
                                    }
                                    break;
                                }
                                case "hgt": {
                                    if (suffix.Substring(suffix.Length - 2) == "in") {
                                        if (numericSuffix >= 150 && numericSuffix <= 193) {
                                            Console.WriteLine("hit3");
                                            containsHgt = true;
                                        }
                                    } else if (suffix.Substring(suffix.Length - 2) == "cm") {
                                        if (numericSuffix >= 59 && numericSuffix <= 76) {
                                            Console.WriteLine("hit4");
                                            containsHgt = true;
                                        }
                                    }
                                    break;
                                }
                                case "hcl": {
                                    bool isValid = true;
                                    foreach (char currentChar in suffix.ToCharArray()) {
                                        if (currentChar != '#') {
                                            if (currentChar > 'f') {
                                                isValid = false;
                                            }
                                        }
                                    }
                                    if (isValid) {
                                        containsHcl = true;
                                    }
                                    break;
                                }
                                case "ecl": {
                                    if (suffix == "amb" || suffix == "blu" || suffix == "brn" || suffix == "gry" || suffix == "grn" || suffix == "hzl" || suffix == "oth") {
                                        containsEcl = true;
                                    }
                                    break;
                                }
                                case "pid": {
                                    bool isValid = true;
                                    if (suffix.Length == 9) {
                                        foreach (char currentChar in suffix.ToCharArray()) {
                                            if (currentChar < '0' || currentChar > 9) {
                                                isValid = false;
                                            }
                                        }
                                        if (isValid) {
                                            Console.WriteLine("hit7");
                                            containsPid = true;                                   
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
                if (containsByr && containsEcl && containsEyr && containsHcl && containsHgt && containsIyr && containsPid) {
                    ++validCounter;
                }
            }
            Console.WriteLine(validCounter);
            Console.Read();
        }
    }
}
