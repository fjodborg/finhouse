# finhouse


## Known Issues 

- [ ] Fix state saving on exit on chrome. 
    - Currently chrome only saves the state every 30 seconds, however i should also save on shutdown/refresh like firefox does.

## Roadmap

- [ ] Create parameter side bar.
    - [ ] Housing, mortgage, interest etc
    - [ ] Payments, tax deduction, etc
    - [ ] Other expenses.
- [x] Create top bar with tabs for each entry.
- [ ] Add dark and light mode.
- [ ] Add plot for value over time.
- [ ] Add readme/tooltip/popup about controls.
- [ ] Remove dependency on server being online (Websocket is running).
- [ ] Support multiple languages.
- [ ] Find a way to avoid double maintenance with the sidebar widgets.
    - Perhaps base it off a json file. This also makes multiple languages easier.