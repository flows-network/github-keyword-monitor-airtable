# <p align="center">Save GitHub issues mentioning a specific keyword to Airtable</p>

<p align="center">
  <a href="https://discord.gg/ccZn9ZMfFf">
    <img src="https://img.shields.io/badge/chat-Discord-7289DA?logo=discord" alt="flows.network Discord">
  </a>
  <a href="https://twitter.com/flows_network">
    <img src="https://img.shields.io/badge/Twitter-1DA1F2?logo=twitter&amp;logoColor=white" alt="flows.network Twitter">
  </a>
   <a href="https://flows.network/flow/createByTemplate/github-keyword-monitor-airtable">
    <img src="https://img.shields.io/website?up_message=deploy&url=https%3A%2F%2Fflows.network%2Fflow%2Fnew" alt="Create a flow">
  </a>
</p>

[Deploy this function on flows.network](https://flows.network/flow/createByTemplate/github-keyword-monitor-airtable), to save GitHub issues that mention your pre-set keyword to an Airtable table. For example, if you set the keyword as WasmEdge, any GitHub issues include WasmEdge will be saved to the Airtable table.

<img width="600" alt="image" src="https://github.com/flows-network/github-keyword-monitor-airtable/assets/45785633/a213af26-c86a-4cbd-bce3-6b2055eae307">


## How it works

This flow function is triggered by time. At the specified time, the flow function searches for all GitHub issues from the past 24 hours, filters them based on your chosen keyword and saves the results into an Airtable table.

## Deploy your own code review flow function in 3 simple steps

1. Create a flow from a template
2. Configure the flow to access Airtable

### 0 Prerequisites

Sign into [flows.network](https://flows.network/) from your GitHub account. It is free.

[A pre-built Airtable table](https://airtable.com/invite/l?inviteId=invN9XkuZjUpVoKZ8&inviteToken=a9c5477d82e73c73b0e5228cdb4083434be061d115e7e88b89ee9df1b3ff4d8b&utm_medium=email&utm_source=product_team&utm_content=transactional-alerts) to save the results.

### 1 Create a flow function from a template

[**Click here to deploy**](https://flows.network/flow/createByTemplate/github-keyword-monitor-airtable)

<img width="500" alt="image" src="https://github.com/flows-network/github-keyword-monitor-airtable/assets/45785633/d5f06598-13fb-4692-b7fa-3712d7ac181e">


Review the `keyword` variable. Enter any keyword that you want to track. Only support one word here.

Click on the **Create and Build** button.

### 2 Configure the flow to access Airtable

Set up Airtable integration. 

<img width="500" alt="image" src="https://github.com/flows-network/github-keyword-monitor-airtable/assets/45785633/b8cf5fd5-a529-45ed-97b6-fc8d83b0ec53">

Enter these 3 parameters. Click [How to connect Airtable](https://flows.network/blog/airtable) for more information.

* airtable_table_name: the name of the table you want to add data, which you can find on Airtable.
* airtable_base_id: the base id to which the table you wish to add or retrieve data belongs, which you can find on Airtable.
* airtable_token_name: the name of the Airtable API token, which is defined by you

Then, click on the **Connect** or **+ Add new authentication** button, enter your key, and give it a name.

Close the tab and go back to the flow.network page once you are done. Click on **Deploy**.

### Wait for the magic!

This is it! You are now on the flow details page waiting for the flow function to build. As soon as the flow's status became `running`, the flow now starts to monitor all the GitHub issues! 


