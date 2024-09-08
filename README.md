# finhouse

## Usage

* You can use suffixes K(1,000) and M(1,000,000) to specify units. If not specified it uses the default one.
* Each tab has its own set of parameters you can customize.
* Right click on tabs to remove them.
* Black boxes (Darkmode) can be edited to custom text.

## Known Issues 

- [ ] Fix state saving on exit on chrome. 
    - Currently chrome only saves the state every 30 seconds, however i should also save on shutdown/refresh like firefox does.

## Roadmap

- [X] Create parameter side bar.
    - [X] Housing, mortgage, interest etc
    - [X] Payments, tax deduction, etc
    - [X] Other expenses.
- [ ] Add inflation.
- [x] Create top bar with tabs for each entry.
- [ ] Add dark and light mode.
- [X] Add plot for value over time.
    - [ ] Add proper utilities, like checkboxes etc.
    - [ ] Add suffix to numbers.
    - [ ] Add "legend" with minimum payment on the plot for each line.
    - [ ] Add option to "Offset" plot with house value. 
- [ ] Add readme/tooltip/popup about controls.
- [ ] Remove dependency on server being online (Websocket is running).
- [ ] Support multiple languages.
- [ ] Find a way to avoid double maintenance with the sidebar widgets.
    - Perhaps base it off a json file. This also makes multiple languages easier.
- [ ] Add "Kurs tab", i don't know the english world.

## Ideas

- [ ] Remove Percentage type and just use f64 with a custom formatter for egui.