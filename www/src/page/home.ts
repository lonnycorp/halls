import * as component from "@www/component"
import { DEKU_IMAGE, DUST_DOOM_IMAGE, DUST_NUKE_IMAGE, NUKE_BLOCKFORT_IMAGE } from "@www/image"
import { shim } from "@www/shim"
import { Document, node } from "@lonnycorp/htmlforge"

const TITLE = "Hallways"

function createScreenshotGrid(): component.ImageGrid {
    return new component.ImageGrid("756 / 471")
        .image(DEKU_IMAGE.publicPath, "The moon from Majora's mask appearing over Kokiri forest.")
        .image(DUST_DOOM_IMAGE.publicPath, "Doom's Hangar opening up to de_dust2.")
        .image(DUST_NUKE_IMAGE.publicPath, "A shaft linking de_dust2 and de_nuke.")
        .image(NUKE_BLOCKFORT_IMAGE.publicPath, "Blockfort from Mario Kart 64 - now accessible from de_nuke.")
}

function createOverview(): node.Element {
    return new node.Element("section")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "14px")
        .style("width", "100%")
        .child(new component.Heading({ kind: component.Kind.H1, title: "Overview" }))
        .child(new node.Text("Hallways is a browser for 3D environments hosted on the internet. You can visit, explore and seamlessly walk through portals to somewhere else."))
        .child(createScreenshotGrid())
}

function createControlsSection(): node.Element {
    return new node.Element("section")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("gap", "14px")
        .style("width", "100%")
        .child(new component.Heading({ kind: component.Kind.H1, title: "Controls" }))
        .child(new node.Text("Controls can be changed within in-game settings - but default controls are as follows."))
        .child(createControlsTable())
}

function createControlsTable(): node.Buildable {
    return new component.Table(["Control", "Action"])
        .row([
            new node.Text("WASD"),
            new node.Text("Movement"),
        ])
        .row([
            new node.Text("Left Ctrl"),
            new node.Text("Crouch"),
        ])
        .row([
            new node.Text("Space"),
            new node.Text("Jump / Enable floating (double tap) / Disable floating"),
        ])
        .row([
            new node.Text("Tab"),
            new node.Text("Toggle debug console"),
        ])
}

export function home(): string {
    const doc = new Document()

    doc.attribute("lang", "en")
    shim(doc.body)

    doc.head.child(new component.Helmet({ title: TITLE }))

    doc.body
        .child(
            new component.Main()
                .child(
                    new component.Card()
                        .child(new component.Header({ text: TITLE }))
                        .child(new component.Divider())
                        .child(createOverview())
                        .child(new component.Divider())
                        .child(createControlsSection())
                )
        )
        .child(new component.Footer())

    return doc.toString()
}
