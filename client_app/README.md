# Rust Server React JsonForms Web Client

This is based on seed from [JSON Forms](https://jsonforms.io) with React.

- Execute `npm ci` to install the prerequisites. If you want to have the latest released versions use `npm install`.
- Execute `npm start` to start the application.

Start the Rust web-server.  
Browse to http://localhost:3000 to see the application in action.

## File Structure

Let's briefly have a look at the most important files:

- `src/index.tsx` is the entry point of the application. We also customize the Material UI theme to give each control more space.
- `src/AppEx.tsx` is the main app component and makes use of the `JsonForms` component in order to render a form.

The [data schema](src/schema.json) defines the structure of a Task: it contains attributes such as title, description, due date and so on.

The [corresponding UI schema](src/uischema.json) specifies controls for each property and puts them into a vertical layout that in turn contains two horizontal layouts.

## Rendering JSON Forms

JSON Forms is rendered by importing and using the `JsonForms` component and directly handing over the `schema`, `uischema`, `data`, `renderer` and `cell` props. We listen to changes in the form via the `onChange` callback.  
Schema and UISchema are retreived from the Rust server.

## Custom renderers

Please see [our corresponding tutorial](https://jsonforms.io/docs/tutorial) on how to add custom renderers.  
Examples can be found in the Controls sub-folder.
