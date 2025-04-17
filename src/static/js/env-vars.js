document.addEventListener("DOMContentLoaded", function() {
    const addEnvVarButton = document.getElementById("add-env-var");
    const envVarsContainer = document.getElementById("env-vars");
    
    if (addEnvVarButton && envVarsContainer) {
        addEnvVarButton.addEventListener("click", function() {
            const envVarRow = document.createElement("div");
            envVarRow.className = "flex space-x-2";
            envVarRow.innerHTML = 
                "<input type=\"text\" name=\"env_keys[]\" placeholder=\"KEY\" class=\"w-1/3 rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500\">" +
                "<input type=\"text\" name=\"env_values[]\" placeholder=\"value\" class=\"w-2/3 rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500\">" +
                "<button type=\"button\" class=\"remove-env-var text-red-600 hover:text-red-800\">" +
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" class=\"h-5 w-5\" viewBox=\"0 0 20 20\" fill=\"currentColor\">" +
                        "<path fill-rule=\"evenodd\" d=\"M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z\" clip-rule=\"evenodd\" />" +
                    "</svg>" +
                "</button>";
            
            envVarsContainer.appendChild(envVarRow);
            
            // Add event listener to remove button
            envVarRow.querySelector(".remove-env-var").addEventListener("click", function() {
                envVarsContainer.removeChild(envVarRow);
            });
        });
    }
});