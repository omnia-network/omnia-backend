export const handleError = (e: Error | unknown) => {

  console.error(e);
  
  alert(e);
};