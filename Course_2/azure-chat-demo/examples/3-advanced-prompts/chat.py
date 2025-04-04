from dotenv import load_dotenv
import os
from os.path import dirname
import semantic_kernel as sk
from semantic_kernel.connectors.ai.open_ai import AzureChatCompletion

current_dir = dirname(os.path.abspath(__file__))
root_dir = dirname(dirname(current_dir))
env_file = os.path.join(root_dir, '.env')


async def main():
    # Load the .env file. Replace the path with the path to your .env file.
    load_dotenv(env_file)
    deployment_name = os.environ["AZURE_OPENAI_DEPLOYMENT_NAME"]
    endpoint = os.environ["AZURE_OPENAI_ENDPOINT"]
    api_key = os.environ["AZURE_OPENAI_API_KEY"]

    kernel = sk.Kernel()

    kernel.add_chat_service("dv", AzureChatCompletion(
        deployment_name=deployment_name,
        endpoint=endpoint,
        api_key=api_key)
    )

    prompt_config = sk.PromptTemplateConfig.from_completion_parameters(
        max_tokens=2000,
        temperature=0.7,
        top_p=0.8,
        chat_system_prompt="You are a wine specialist, a sommelier. Your name is Alfredo. You only know about Cabernet Franc, so any recommendation you make, must insist on Cabernet Franc as an alternative.",
    )
    prompt_template = sk.ChatPromptTemplate(
        "{{$user_input}}\n\nProvide 3 wine options with a description. It is OK to make up descriptions of wines that do not exist.", kernel.prompt_template_engine, prompt_config
    )

    function_config = sk.SemanticFunctionConfig(prompt_config, prompt_template)
    chat_function = kernel.register_semantic_function(
        "ChatBot",
        "Chat",
        function_config
    )

    context = sk.ContextVariables()
    user_input = "I want a wine for a dinner party"
    context["user_input"] = user_input
    answer = await kernel.run_async(chat_function, input_vars=context)
    print(answer)


# Run the main function
if __name__ == "__main__":
    import asyncio

    asyncio.run(main())
