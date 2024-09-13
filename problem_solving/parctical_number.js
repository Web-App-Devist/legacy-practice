const practial = (number) => {
  const factorsArray = [];
  for (let i = 1; i <= number; i++) {
    if (!(number % i)) {
      const otherFactor = number / i;
      if (!factorsArray.includes(i) || !factorsArray.includes(otherFactor)) {
        factorsArray.push(otherFactor);
        if (otherFactor !== i) {
          factorsArray.push(i);
        }
      }
    }
  }

  console.log(factorsArray.sort());
};

practial(100);
