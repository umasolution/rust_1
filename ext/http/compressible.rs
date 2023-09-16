// Copyright 2018-2023 the Deno authors. All rights reserved. MIT license.

use phf::phf_set;
use std::str::FromStr;

// Data obtained from https://github.com/jshttp/mime-db/blob/fa5e4ef3cc8907ec3c5ec5b85af0c63d7059a5cd/db.json
// Important! Keep this list sorted alphabetically.
static CONTENT_TYPES: phf::Set<&'static [u8]> = phf_set! {
  b"application/3gpdash-qoe-report+xml",
  b"application/3gpp-ims+xml",
  b"application/3gpphal+json",
  b"application/3gpphalforms+json",
  b"application/activity+json",
  b"application/alto-costmap+json",
  b"application/alto-costmapfilter+json",
  b"application/alto-directory+json",
  b"application/alto-endpointcost+json",
  b"application/alto-endpointcostparams+json",
  b"application/alto-endpointprop+json",
  b"application/alto-endpointpropparams+json",
  b"application/alto-error+json",
  b"application/alto-networkmap+json",
  b"application/alto-networkmapfilter+json",
  b"application/alto-updatestreamcontrol+json",
  b"application/alto-updatestreamparams+json",
  b"application/atom+xml",
  b"application/atomcat+xml",
  b"application/atomdeleted+xml",
  b"application/atomsvc+xml",
  b"application/atsc-dwd+xml",
  b"application/atsc-held+xml",
  b"application/atsc-rdt+json",
  b"application/atsc-rsat+xml",
  b"application/auth-policy+xml",
  b"application/beep+xml",
  b"application/calendar+json",
  b"application/calendar+xml",
  b"application/captive+json",
  b"application/ccmp+xml",
  b"application/ccxml+xml",
  b"application/cdfx+xml",
  b"application/cea-2018+xml",
  b"application/cellml+xml",
  b"application/clue+xml",
  b"application/clue_info+xml",
  b"application/cnrp+xml",
  b"application/coap-group+json",
  b"application/conference-info+xml",
  b"application/cpl+xml",
  b"application/csta+xml",
  b"application/cstadata+xml",
  b"application/csvm+json",
  b"application/dart",
  b"application/dash+xml",
  b"application/davmount+xml",
  b"application/dialog-info+xml",
  b"application/dicom+json",
  b"application/dicom+xml",
  b"application/dns+json",
  b"application/docbook+xml",
  b"application/dskpp+xml",
  b"application/dssc+xml",
  b"application/ecmascript",
  b"application/elm+json",
  b"application/elm+xml",
  b"application/emergencycalldata.cap+xml",
  b"application/emergencycalldata.comment+xml",
  b"application/emergencycalldata.control+xml",
  b"application/emergencycalldata.deviceinfo+xml",
  b"application/emergencycalldata.providerinfo+xml",
  b"application/emergencycalldata.serviceinfo+xml",
  b"application/emergencycalldata.subscriberinfo+xml",
  b"application/emergencycalldata.veds+xml",
  b"application/emma+xml",
  b"application/emotionml+xml",
  b"application/epp+xml",
  b"application/expect-ct-report+json",
  b"application/fdt+xml",
  b"application/fhir+json",
  b"application/fhir+xml",
  b"application/fido.trusted-apps+json",
  b"application/framework-attributes+xml",
  b"application/geo+json",
  b"application/geoxacml+xml",
  b"application/gml+xml",
  b"application/gpx+xml",
  b"application/held+xml",
  b"application/ibe-key-request+xml",
  b"application/ibe-pkg-reply+xml",
  b"application/im-iscomposing+xml",
  b"application/inkml+xml",
  b"application/its+xml",
  b"application/javascript",
  b"application/jf2feed+json",
  b"application/jose+json",
  b"application/jrd+json",
  b"application/jscalendar+json",
  b"application/json",
  b"application/json-patch+json",
  b"application/jsonml+json",
  b"application/jwk+json",
  b"application/jwk-set+json",
  b"application/kpml-request+xml",
  b"application/kpml-response+xml",
  b"application/ld+json",
  b"application/lgr+xml",
  b"application/load-control+xml",
  b"application/lost+xml",
  b"application/lostsync+xml",
  b"application/mads+xml",
  b"application/manifest+json",
  b"application/marcxml+xml",
  b"application/mathml+xml",
  b"application/mathml-content+xml",
  b"application/mathml-presentation+xml",
  b"application/mbms-associated-procedure-description+xml",
  b"application/mbms-deregister+xml",
  b"application/mbms-envelope+xml",
  b"application/mbms-msk+xml",
  b"application/mbms-msk-response+xml",
  b"application/mbms-protection-description+xml",
  b"application/mbms-reception-report+xml",
  b"application/mbms-register+xml",
  b"application/mbms-register-response+xml",
  b"application/mbms-schedule+xml",
  b"application/mbms-user-service-description+xml",
  b"application/media-policy-dataset+xml",
  b"application/media_control+xml",
  b"application/mediaservercontrol+xml",
  b"application/merge-patch+json",
  b"application/metalink+xml",
  b"application/metalink4+xml",
  b"application/mets+xml",
  b"application/mmt-aei+xml",
  b"application/mmt-usd+xml",
  b"application/mods+xml",
  b"application/mrb-consumer+xml",
  b"application/mrb-publish+xml",
  b"application/msc-ivr+xml",
  b"application/msc-mixer+xml",
  b"application/mud+json",
  b"application/nlsml+xml",
  b"application/odm+xml",
  b"application/oebps-package+xml",
  b"application/omdoc+xml",
  b"application/opc-nodeset+xml",
  b"application/p2p-overlay+xml",
  b"application/patch-ops-error+xml",
  b"application/pidf+xml",
  b"application/pidf-diff+xml",
  b"application/pls+xml",
  b"application/poc-settings+xml",
  b"application/postscript",
  b"application/ppsp-tracker+json",
  b"application/problem+json",
  b"application/problem+xml",
  b"application/provenance+xml",
  b"application/prs.xsf+xml",
  b"application/pskc+xml",
  b"application/pvd+json",
  b"application/raml+yaml",
  b"application/rdap+json",
  b"application/rdf+xml",
  b"application/reginfo+xml",
  b"application/reputon+json",
  b"application/resource-lists+xml",
  b"application/resource-lists-diff+xml",
  b"application/rfc+xml",
  b"application/rlmi+xml",
  b"application/rls-services+xml",
  b"application/route-apd+xml",
  b"application/route-s-tsid+xml",
  b"application/route-usd+xml",
  b"application/rsd+xml",
  b"application/rss+xml",
  b"application/rtf",
  b"application/samlassertion+xml",
  b"application/samlmetadata+xml",
  b"application/sarif+json",
  b"application/sarif-external-properties+json",
  b"application/sbml+xml",
  b"application/scaip+xml",
  b"application/scim+json",
  b"application/senml+json",
  b"application/senml+xml",
  b"application/senml-etch+json",
  b"application/sensml+json",
  b"application/sensml+xml",
  b"application/sep+xml",
  b"application/shf+xml",
  b"application/simple-filter+xml",
  b"application/smil+xml",
  b"application/soap+xml",
  b"application/sparql-results+xml",
  b"application/spirits-event+xml",
  b"application/srgs+xml",
  b"application/sru+xml",
  b"application/ssdl+xml",
  b"application/ssml+xml",
  b"application/stix+json",
  b"application/swid+xml",
  b"application/tar",
  b"application/taxii+json",
  b"application/td+json",
  b"application/tei+xml",
  b"application/thraud+xml",
  b"application/tlsrpt+json",
  b"application/toml",
  b"application/ttml+xml",
  b"application/urc-grpsheet+xml",
  b"application/urc-ressheet+xml",
  b"application/urc-targetdesc+xml",
  b"application/urc-uisocketdesc+xml",
  b"application/vcard+json",
  b"application/vcard+xml",
  b"application/vnd.1000minds.decision-model+xml",
  b"application/vnd.3gpp-prose+xml",
  b"application/vnd.3gpp-prose-pc3ch+xml",
  b"application/vnd.3gpp.access-transfer-events+xml",
  b"application/vnd.3gpp.bsf+xml",
  b"application/vnd.3gpp.gmop+xml",
  b"application/vnd.3gpp.mcdata-affiliation-command+xml",
  b"application/vnd.3gpp.mcdata-info+xml",
  b"application/vnd.3gpp.mcdata-service-config+xml",
  b"application/vnd.3gpp.mcdata-ue-config+xml",
  b"application/vnd.3gpp.mcdata-user-profile+xml",
  b"application/vnd.3gpp.mcptt-affiliation-command+xml",
  b"application/vnd.3gpp.mcptt-floor-request+xml",
  b"application/vnd.3gpp.mcptt-info+xml",
  b"application/vnd.3gpp.mcptt-location-info+xml",
  b"application/vnd.3gpp.mcptt-mbms-usage-info+xml",
  b"application/vnd.3gpp.mcptt-service-config+xml",
  b"application/vnd.3gpp.mcptt-signed+xml",
  b"application/vnd.3gpp.mcptt-ue-config+xml",
  b"application/vnd.3gpp.mcptt-ue-init-config+xml",
  b"application/vnd.3gpp.mcptt-user-profile+xml",
  b"application/vnd.3gpp.mcvideo-affiliation-command+xml",
  b"application/vnd.3gpp.mcvideo-affiliation-info+xml",
  b"application/vnd.3gpp.mcvideo-info+xml",
  b"application/vnd.3gpp.mcvideo-location-info+xml",
  b"application/vnd.3gpp.mcvideo-mbms-usage-info+xml",
  b"application/vnd.3gpp.mcvideo-service-config+xml",
  b"application/vnd.3gpp.mcvideo-transmission-request+xml",
  b"application/vnd.3gpp.mcvideo-ue-config+xml",
  b"application/vnd.3gpp.mcvideo-user-profile+xml",
  b"application/vnd.3gpp.mid-call+xml",
  b"application/vnd.3gpp.sms+xml",
  b"application/vnd.3gpp.srvcc-ext+xml",
  b"application/vnd.3gpp.srvcc-info+xml",
  b"application/vnd.3gpp.state-and-event-info+xml",
  b"application/vnd.3gpp.ussd+xml",
  b"application/vnd.3gpp2.bcmcsinfo+xml",
  b"application/vnd.adobe.xdp+xml",
  b"application/vnd.amadeus+json",
  b"application/vnd.amundsen.maze+xml",
  b"application/vnd.api+json",
  b"application/vnd.aplextor.warrp+json",
  b"application/vnd.apothekende.reservation+json",
  b"application/vnd.apple.installer+xml",
  b"application/vnd.artisan+json",
  b"application/vnd.avalon+json",
  b"application/vnd.avistar+xml",
  b"application/vnd.balsamiq.bmml+xml",
  b"application/vnd.bbf.usp.msg+json",
  b"application/vnd.bekitzur-stech+json",
  b"application/vnd.biopax.rdf+xml",
  b"application/vnd.byu.uapi+json",
  b"application/vnd.capasystems-pg+json",
  b"application/vnd.chemdraw+xml",
  b"application/vnd.citationstyles.style+xml",
  b"application/vnd.collection+json",
  b"application/vnd.collection.doc+json",
  b"application/vnd.collection.next+json",
  b"application/vnd.coreos.ignition+json",
  b"application/vnd.criticaltools.wbs+xml",
  b"application/vnd.cryptii.pipe+json",
  b"application/vnd.ctct.ws+xml",
  b"application/vnd.cyan.dean.root+xml",
  b"application/vnd.cyclonedx+json",
  b"application/vnd.cyclonedx+xml",
  b"application/vnd.dart",
  b"application/vnd.datapackage+json",
  b"application/vnd.dataresource+json",
  b"application/vnd.dece.ttml+xml",
  b"application/vnd.dm.delegation+xml",
  b"application/vnd.document+json",
  b"application/vnd.drive+json",
  b"application/vnd.dvb.dvbisl+xml",
  b"application/vnd.dvb.notif-aggregate-root+xml",
  b"application/vnd.dvb.notif-container+xml",
  b"application/vnd.dvb.notif-generic+xml",
  b"application/vnd.dvb.notif-ia-msglist+xml",
  b"application/vnd.dvb.notif-ia-registration-request+xml",
  b"application/vnd.dvb.notif-ia-registration-response+xml",
  b"application/vnd.dvb.notif-init+xml",
  b"application/vnd.emclient.accessrequest+xml",
  b"application/vnd.eprints.data+xml",
  b"application/vnd.eszigno3+xml",
  b"application/vnd.etsi.aoc+xml",
  b"application/vnd.etsi.cug+xml",
  b"application/vnd.etsi.iptvcommand+xml",
  b"application/vnd.etsi.iptvdiscovery+xml",
  b"application/vnd.etsi.iptvprofile+xml",
  b"application/vnd.etsi.iptvsad-bc+xml",
  b"application/vnd.etsi.iptvsad-cod+xml",
  b"application/vnd.etsi.iptvsad-npvr+xml",
  b"application/vnd.etsi.iptvservice+xml",
  b"application/vnd.etsi.iptvsync+xml",
  b"application/vnd.etsi.iptvueprofile+xml",
  b"application/vnd.etsi.mcid+xml",
  b"application/vnd.etsi.overload-control-policy-dataset+xml",
  b"application/vnd.etsi.pstn+xml",
  b"application/vnd.etsi.sci+xml",
  b"application/vnd.etsi.simservs+xml",
  b"application/vnd.etsi.tsl+xml",
  b"application/vnd.fujifilm.fb.jfi+xml",
  b"application/vnd.futoin+json",
  b"application/vnd.gentics.grd+json",
  b"application/vnd.geo+json",
  b"application/vnd.geocube+xml",
  b"application/vnd.google-earth.kml+xml",
  b"application/vnd.gov.sk.e-form+xml",
  b"application/vnd.gov.sk.xmldatacontainer+xml",
  b"application/vnd.hal+json",
  b"application/vnd.hal+xml",
  b"application/vnd.handheld-entertainment+xml",
  b"application/vnd.hc+json",
  b"application/vnd.heroku+json",
  b"application/vnd.hyper+json",
  b"application/vnd.hyper-item+json",
  b"application/vnd.hyperdrive+json",
  b"application/vnd.ims.lis.v2.result+json",
  b"application/vnd.ims.lti.v2.toolconsumerprofile+json",
  b"application/vnd.ims.lti.v2.toolproxy+json",
  b"application/vnd.ims.lti.v2.toolproxy.id+json",
  b"application/vnd.ims.lti.v2.toolsettings+json",
  b"application/vnd.ims.lti.v2.toolsettings.simple+json",
  b"application/vnd.informedcontrol.rms+xml",
  b"application/vnd.infotech.project+xml",
  b"application/vnd.iptc.g2.catalogitem+xml",
  b"application/vnd.iptc.g2.conceptitem+xml",
  b"application/vnd.iptc.g2.knowledgeitem+xml",
  b"application/vnd.iptc.g2.newsitem+xml",
  b"application/vnd.iptc.g2.newsmessage+xml",
  b"application/vnd.iptc.g2.packageitem+xml",
  b"application/vnd.iptc.g2.planningitem+xml",
  b"application/vnd.irepository.package+xml",
  b"application/vnd.las.las+json",
  b"application/vnd.las.las+xml",
  b"application/vnd.leap+json",
  b"application/vnd.liberty-request+xml",
  b"application/vnd.llamagraphics.life-balance.exchange+xml",
  b"application/vnd.marlin.drm.actiontoken+xml",
  b"application/vnd.marlin.drm.conftoken+xml",
  b"application/vnd.marlin.drm.license+xml",
  b"application/vnd.mason+json",
  b"application/vnd.micro+json",
  b"application/vnd.miele+json",
  b"application/vnd.mozilla.xul+xml",
  b"application/vnd.ms-fontobject",
  b"application/vnd.ms-office.activex+xml",
  b"application/vnd.ms-opentype",
  b"application/vnd.ms-playready.initiator+xml",
  b"application/vnd.ms-printdevicecapabilities+xml",
  b"application/vnd.ms-printing.printticket+xml",
  b"application/vnd.ms-printschematicket+xml",
  b"application/vnd.nearst.inv+json",
  b"application/vnd.nokia.conml+xml",
  b"application/vnd.nokia.iptv.config+xml",
  b"application/vnd.nokia.landmark+xml",
  b"application/vnd.nokia.landmarkcollection+xml",
  b"application/vnd.nokia.n-gage.ac+xml",
  b"application/vnd.nokia.pcd+xml",
  b"application/vnd.oci.image.manifest.v1+json",
  b"application/vnd.oftn.l10n+json",
  b"application/vnd.oipf.contentaccessdownload+xml",
  b"application/vnd.oipf.contentaccessstreaming+xml",
  b"application/vnd.oipf.dae.svg+xml",
  b"application/vnd.oipf.dae.xhtml+xml",
  b"application/vnd.oipf.mippvcontrolmessage+xml",
  b"application/vnd.oipf.spdiscovery+xml",
  b"application/vnd.oipf.spdlist+xml",
  b"application/vnd.oipf.ueprofile+xml",
  b"application/vnd.oipf.userprofile+xml",
  b"application/vnd.oma.bcast.associated-procedure-parameter+xml",
  b"application/vnd.oma.bcast.drm-trigger+xml",
  b"application/vnd.oma.bcast.imd+xml",
  b"application/vnd.oma.bcast.notification+xml",
  b"application/vnd.oma.bcast.sgdd+xml",
  b"application/vnd.oma.bcast.smartcard-trigger+xml",
  b"application/vnd.oma.bcast.sprov+xml",
  b"application/vnd.oma.cab-address-book+xml",
  b"application/vnd.oma.cab-feature-handler+xml",
  b"application/vnd.oma.cab-pcc+xml",
  b"application/vnd.oma.cab-subs-invite+xml",
  b"application/vnd.oma.cab-user-prefs+xml",
  b"application/vnd.oma.dd2+xml",
  b"application/vnd.oma.drm.risd+xml",
  b"application/vnd.oma.group-usage-list+xml",
  b"application/vnd.oma.lwm2m+json",
  b"application/vnd.oma.pal+xml",
  b"application/vnd.oma.poc.detailed-progress-report+xml",
  b"application/vnd.oma.poc.final-report+xml",
  b"application/vnd.oma.poc.groups+xml",
  b"application/vnd.oma.poc.invocation-descriptor+xml",
  b"application/vnd.oma.poc.optimized-progress-report+xml",
  b"application/vnd.oma.scidm.messages+xml",
  b"application/vnd.oma.xcap-directory+xml",
  b"application/vnd.omads-email+xml",
  b"application/vnd.omads-file+xml",
  b"application/vnd.omads-folder+xml",
  b"application/vnd.openblox.game+xml",
  b"application/vnd.openstreetmap.data+xml",
  b"application/vnd.openxmlformats-officedocument.custom-properties+xml",
  b"application/vnd.openxmlformats-officedocument.customxmlproperties+xml",
  b"application/vnd.openxmlformats-officedocument.drawing+xml",
  b"application/vnd.openxmlformats-officedocument.drawingml.chart+xml",
  b"application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
  b"application/vnd.openxmlformats-officedocument.drawingml.diagramcolors+xml",
  b"application/vnd.openxmlformats-officedocument.drawingml.diagramdata+xml",
  b"application/vnd.openxmlformats-officedocument.drawingml.diagramlayout+xml",
  b"application/vnd.openxmlformats-officedocument.drawingml.diagramstyle+xml",
  b"application/vnd.openxmlformats-officedocument.extended-properties+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.commentauthors+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.comments+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.handoutmaster+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.notesmaster+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.notesslide+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.presprops+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.slide+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.slidelayout+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.slidemaster+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.slideupdateinfo+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.tablestyles+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.tags+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.template.main+xml",
  b"application/vnd.openxmlformats-officedocument.presentationml.viewprops+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.calcchain+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.externallink+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.pivotcachedefinition+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.pivotcacherecords+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.pivottable+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.querytable+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.revisionheaders+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.revisionlog+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedstrings+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.sheetmetadata+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.tablesinglecells+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.usernames+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.volatiledependencies+xml",
  b"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
  b"application/vnd.openxmlformats-officedocument.theme+xml",
  b"application/vnd.openxmlformats-officedocument.themeoverride+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.fonttable+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml",
  b"application/vnd.openxmlformats-officedocument.wordprocessingml.websettings+xml",
  b"application/vnd.openxmlformats-package.core-properties+xml",
  b"application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml",
  b"application/vnd.openxmlformats-package.relationships+xml",
  b"application/vnd.oracle.resource+json",
  b"application/vnd.otps.ct-kip+xml",
  b"application/vnd.pagerduty+json",
  b"application/vnd.poc.group-advertisement+xml",
  b"application/vnd.pwg-xhtml-print+xml",
  b"application/vnd.radisys.moml+xml",
  b"application/vnd.radisys.msml+xml",
  b"application/vnd.radisys.msml-audit+xml",
  b"application/vnd.radisys.msml-audit-conf+xml",
  b"application/vnd.radisys.msml-audit-conn+xml",
  b"application/vnd.radisys.msml-audit-dialog+xml",
  b"application/vnd.radisys.msml-audit-stream+xml",
  b"application/vnd.radisys.msml-conf+xml",
  b"application/vnd.radisys.msml-dialog+xml",
  b"application/vnd.radisys.msml-dialog-base+xml",
  b"application/vnd.radisys.msml-dialog-fax-detect+xml",
  b"application/vnd.radisys.msml-dialog-fax-sendrecv+xml",
  b"application/vnd.radisys.msml-dialog-group+xml",
  b"application/vnd.radisys.msml-dialog-speech+xml",
  b"application/vnd.radisys.msml-dialog-transform+xml",
  b"application/vnd.recordare.musicxml+xml",
  b"application/vnd.restful+json",
  b"application/vnd.route66.link66+xml",
  b"application/vnd.seis+json",
  b"application/vnd.shootproof+json",
  b"application/vnd.shopkick+json",
  b"application/vnd.siren+json",
  b"application/vnd.software602.filler.form+xml",
  b"application/vnd.solent.sdkm+xml",
  b"application/vnd.sun.wadl+xml",
  b"application/vnd.sycle+xml",
  b"application/vnd.syncml+xml",
  b"application/vnd.syncml.dm+xml",
  b"application/vnd.syncml.dmddf+xml",
  b"application/vnd.syncml.dmtnds+xml",
  b"application/vnd.tableschema+json",
  b"application/vnd.think-cell.ppttc+json",
  b"application/vnd.tmd.mediaflex.api+xml",
  b"application/vnd.uoml+xml",
  b"application/vnd.vel+json",
  b"application/vnd.wv.csp+xml",
  b"application/vnd.wv.ssp+xml",
  b"application/vnd.xacml+json",
  b"application/vnd.xmi+xml",
  b"application/vnd.yamaha.openscoreformat.osfpvg+xml",
  b"application/vnd.zzazz.deck+xml",
  b"application/voicexml+xml",
  b"application/voucher-cms+json",
  b"application/wasm",
  b"application/watcherinfo+xml",
  b"application/webpush-options+json",
  b"application/wsdl+xml",
  b"application/wspolicy+xml",
  b"application/x-dtbncx+xml",
  b"application/x-dtbook+xml",
  b"application/x-dtbresource+xml",
  b"application/x-httpd-php",
  b"application/x-javascript",
  b"application/x-ns-proxy-autoconfig",
  b"application/x-sh",
  b"application/x-tar",
  b"application/x-virtualbox-hdd",
  b"application/x-virtualbox-ova",
  b"application/x-virtualbox-ovf",
  b"application/x-virtualbox-vbox",
  b"application/x-virtualbox-vdi",
  b"application/x-virtualbox-vhd",
  b"application/x-virtualbox-vmdk",
  b"application/x-web-app-manifest+json",
  b"application/x-www-form-urlencoded",
  b"application/x-xliff+xml",
  b"application/xacml+xml",
  b"application/xaml+xml",
  b"application/xcap-att+xml",
  b"application/xcap-caps+xml",
  b"application/xcap-diff+xml",
  b"application/xcap-el+xml",
  b"application/xcap-error+xml",
  b"application/xcap-ns+xml",
  b"application/xcon-conference-info+xml",
  b"application/xcon-conference-info-diff+xml",
  b"application/xenc+xml",
  b"application/xhtml+xml",
  b"application/xhtml-voice+xml",
  b"application/xliff+xml",
  b"application/xml",
  b"application/xml-dtd",
  b"application/xml-patch+xml",
  b"application/xmpp+xml",
  b"application/xop+xml",
  b"application/xproc+xml",
  b"application/xslt+xml",
  b"application/xspf+xml",
  b"application/xv+xml",
  b"application/yang-data+json",
  b"application/yang-data+xml",
  b"application/yang-patch+json",
  b"application/yang-patch+xml",
  b"application/yin+xml",
  b"font/otf",
  b"font/ttf",
  b"image/bmp",
  b"image/svg+xml",
  b"image/vnd.adobe.photoshop",
  b"image/x-icon",
  b"image/x-ms-bmp",
  b"message/imdn+xml",
  b"message/rfc822",
  b"model/gltf+json",
  b"model/gltf-binary",
  b"model/vnd.collada+xml",
  b"model/vnd.moml+xml",
  b"model/x3d+xml",
  b"text/cache-manifest",
  b"text/calender",
  b"text/cmd",
  b"text/css",
  b"text/csv",
  b"text/html",
  b"text/javascript",
  b"text/jsx",
  b"text/less",
  b"text/markdown",
  b"text/mdx",
  b"text/n3",
  b"text/plain",
  b"text/richtext",
  b"text/rtf",
  b"text/tab-separated-values",
  b"text/uri-list",
  b"text/vcard",
  b"text/vtt",
  b"text/x-gwt-rpc",
  b"text/x-jquery-tmpl",
  b"text/x-markdown",
  b"text/x-org",
  b"text/x-processing",
  b"text/x-suse-ymp",
  b"text/xml",
  b"text/yaml",
  b"x-shader/x-fragment",
  b"x-shader/x-vertex",
};

fn known_compressible(ct: &[u8]) -> bool {
  CONTENT_TYPES.contains(ct)
}

fn known_mime(ct: &[u8]) -> Option<bool> {
  let s = std::str::from_utf8(ct).ok()?;
  let m = mime::Mime::from_str(s).ok()?;
  Some(known_compressible(m.essence_str().as_bytes()))
}

/// Determine if the supplied content type is considered compressible
pub fn is_content_compressible(ct: impl AsRef<[u8]>) -> bool {
  let ct = ct.as_ref();
  let prefix = ct.split(|c| *c == b';').next().unwrap();
  known_compressible(prefix) || known_mime(prefix).unwrap_or_default()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn non_compressible_content_type() {
    assert!(!is_content_compressible("application/vnd.deno+json"));
    assert!(!is_content_compressible("text/fake"));
  }

  #[test]
  fn compressible_content_type() {
    assert!(is_content_compressible("application/json"));
    assert!(is_content_compressible("text/plain;charset=UTF-8"));
    assert!(is_content_compressible("text/PlAIn; charset=utf-8"));
  }
}
