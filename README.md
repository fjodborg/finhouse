# finhouse

This is just a hobby project where i play around with different rust ideas. The purpose is to get a better overview of taking a loan for housing.

A demo can be seen here: [Demo](https://fjodborg.github.io/finhouse_page/)

## Usage

* You can use suffixes K(1,000) and M(1,000,000) to specify units. If not specified it uses the default one, meaning the one specified in the field before writing.
* Some value depend on the years specified above the plot and some depend on the specific entry.
* Each tab has its own set of parameters you can customize.
* Right click on tabs to remove them.
* Black boxes (Darkmode) can be edited to custom text.
* Parameters are cached on your pc.
    * Remember if you use chrome and maybe chromium based browsers and you don't want to loose your parameters wait 10 seconds before refreshing/closing it. See __known issues__ for more info. This is not a problem for firefox. 
    * Cache is reset if some specific parts of the code is changed and it get's redeployed. 

## Running the application

### Web
You might need to install _trunk_ binary and _wasm_ targets
`trunk serve --release`

### Native

`cargo run --release`

You can optionally specify the target for the binary, that way it speeds up the compile time for native by a lot.

## Known Issues 

- [ ] Fix state saving on exit on chrome. 
    - Currently chrome only saves the state every 10 seconds, however it should also save on shutdown/refresh like firefox does.
- [ ] Javascript file needs "./" to work with github page.  
- [ ] Cache gets reset when main struct has been modified. 
- [ ] Reference counted Cells are deserialized to default or previous value when loading the page. However they should all reference the same. Workaround is currently implemented.
- [ ] Floating point inaccuracies/fluctuations in current calculations when sliding some parameters. 
- [ ] If you leave in e.g. % or Dkk the field is not accepted. Can be solved by expanding the custom parser.
- [ ] Interest deduction entry is not correct, since it varies over time (Depends on loan), thus should be plotted instead. 

## Roadmap
- [ ] Create regression tests.
- [X] Create parameter side bar.
    - [X] Housing, mortgage, interest etc
    - [X] Payments, tax deduction, etc
    - [X] Other expenses.
- [ ] Add stocks parameters.
    - [ ] Monthly put.
- [ ] Add default with house vs renting and stocks.
- [ ] Add inflation.
- [ ] Add total expenses paid for each expense. (box plot?)
- [ ] Hover over text.
- [x] Create top bar with tabs for each entry.
- [X] Add dark and light mode.
- [X] Add plot for value over time.
    - [X] Add axis text.
    - [ ] Add "legend" with minimum payment on the plot for each line.
    - [ ] Add option to "Offset" plot with house value. 
    - [ ] Add multiple plots, e.g. loan payment, stock, value, total.
- [ ] Add readme/tooltip/popup about controls.
- [ ] Remove dependency on server being online (Websocket is running).
- [ ] Support multiple languages.
- [ ] Find a way to avoid double maintenance with the sidebar widgets.
    - Perhaps base it off a json file. This also makes multiple languages easier.
- [ ] Add "Kurs tab", i don't know the english world.
- [ ] Change font size in plot.
- [ ] Make tabs editable and clickable?
- [ ] Add loan types. e.g. "Realkredit" and "banklån"
- [ ] Make text red if invalid value. e.g. if "Rådighedsbeløb" is negative. 

## Misc

- [ ] Remove Percentage type and just use f64 with a custom formatter for egui.