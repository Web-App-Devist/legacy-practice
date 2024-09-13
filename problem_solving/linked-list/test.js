const debounce = (func, delay) => {
  let timeout = null;
  return (i) => {
    if (timeout) clearTimeout(timeout);

    timeout = setTimeout(() => {
      func(i);
    }, delay);
    5;
  };
};

function throttle(mainFunction, delay) {
  let timerFlag = null; // Variable to keep track of the timer

  // Returning a throttled version
  return (...args) => {
    if (timerFlag === null) {
      // If there is no timer currently running
      mainFunction(...args); // Execute the main function
      timerFlag = setTimeout(() => {
        // Set a timer to clear the timerFlag after the specified delay
        timerFlag = null; // Clear the timerFlag to allow the main function to be executed again
      }, delay);
    }
  };
}
