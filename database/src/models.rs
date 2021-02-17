extern crate serde;
use diesel::Queryable;
use serde::{Deserialize, Serialize};
use crate::{utils::contat_address };
#[derive(Queryable)]
pub struct Hellomaps {
    pub tdg_id_origine: String,
    pub tdg_nom: String,
    pub tdg_adresse: String,
    pub tdg_code_postal: String,
    pub tdg_commune: String,
    pub tdg_code_naf: String,
    pub dcr_com: String,
    pub energie: String,
    pub fct_civilite: String,
    pub fct_nom: String,
    pub fct_prenom: String,
    pub fct_tel_fixe: String,
    pub fct_tel_mobile: String,
    pub fct_email: String,
    pub act_date_creation: String,
    pub act_support: String,
    pub pgr_offre: String,
    pub id: i32,
}
#[derive(Queryable, Serialize, Deserialize)]
pub struct Prospect {
    pub tdg_nom: Option<String>,
    pub tdg_adresse: Option<String>,
    pub tdg_code_postal: Option<String>,
    pub tdg_code_naf: Option<String>,
    pub date_concu: Option<String>,
    pub tdg_id_origine: Option<String>,
    pub tdg_commune: Option<String>,
    pub dcr_com: Option<String>,
    pub energie: Option<String>,
    pub fct_civilite: Option<String>,
    pub fct_nom: Option<String>,
    pub fct_prenom: Option<String>,
    pub fct_tel_fixe: Option<String>,
    pub fct_tel_mobile: Option<String>,
    pub fct_email: Option<String>,
    pub act_date_creation: Option<String>,
    pub act_support: Option<String>,
    pub pgr_offre: Option<String>,
}
#[derive( Serialize, Deserialize)]
pub struct Prospect_full {
    pub tdg_nom: String,
    pub tdg_adresse: String,
    pub tdg_code_postal: String,
    pub tdg_libelle_naf: String,
    pub date_concu: String,
    pub tdg_id_origine: String,
    pub tdg_commune: String,
    pub dcr_com: String,
    pub energie: String,
    pub fct_civilite: String,
    pub fct_nom: String,
    pub fct_prenom: String,
    pub fct_tel_fixe: String,
    pub fct_tel_mobile: String,
    pub fct_email: String,
    pub act_date_creation: String,
    pub act_support: String,
    pub pgr_offre: String,
    pub lat: String,
    pub lng: String,
}
#[derive( Serialize, Deserialize)]
pub struct Prospect_light {
    pub pds_id_rae: String,
    pub tdg_nom: String,
    pub tdg_adresse: String,
    pub tdg_adresse_full: String,
    pub tdg_code_postal: String,
    pub tdg_id_origine: String,
    pub tdg_commune: String,
    pub fct_nom: String,
    pub fct_prenom: String,
}
pub fn unwrapper(data: &Option<String>) -> String {
    match data {
        None => String::from(""),
        Some(data) => String::from(data),
    }
}

impl Prospect {
    pub fn full(&self) -> Prospect_full {
        Prospect_full{
                    tdg_nom: String::from(unwrapper(&self.tdg_nom).as_str()),
                    tdg_adresse: String::from(unwrapper(&self.tdg_adresse).as_str()),
                    tdg_code_postal: String::from(unwrapper(&self.tdg_code_postal).as_str()),
                    date_concu: String::from(unwrapper(&self.date_concu).as_str()),
                    tdg_id_origine: String::from(unwrapper(&self.tdg_id_origine).as_str()),
                    tdg_commune: String::from(unwrapper(&self.tdg_commune).as_str()),
                    dcr_com: String::from(unwrapper(&self.dcr_com).as_str()),
                    energie: String::from(unwrapper(&self.energie).as_str()),
                    fct_civilite: String::from(unwrapper(&self.fct_civilite).as_str()),
                    fct_nom: String::from(unwrapper(&self.fct_nom).as_str()),
                    fct_prenom: String::from(unwrapper(&self.fct_prenom).as_str()),
                    fct_tel_fixe: String::from(unwrapper(&self.fct_tel_fixe).as_str()),
                    fct_tel_mobile: String::from(unwrapper(&self.fct_tel_mobile).as_str()),
                    fct_email: String::from(unwrapper(&self.fct_email).as_str()),
                    act_date_creation: String::from(unwrapper(&self.act_date_creation).as_str()),
                    act_support: String::from(unwrapper(&self.act_support).as_str()),
                    pgr_offre: String::from(unwrapper(&self.pgr_offre).as_str()),
                    tdg_libelle_naf : "".to_string(),
                    lat: "".to_string(),
                    lng: "".to_string(),
        }
    }

    pub fn light(&self) -> Prospect_light {
        let address = String::from(unwrapper(&self.tdg_adresse).as_str());
let city = String::from(unwrapper(&self.tdg_commune).as_str());
        Prospect_light{
                    pds_id_rae: String::from(self.pds_id_rae.as_str()),
                    tdg_nom: String::from(unwrapper(&self.tdg_nom).as_str()),
                    tdg_adresse: String::from(unwrapper(&self.tdg_adresse).as_str()),
                    tdg_code_postal: String::from(unwrapper(&self.tdg_code_postal).as_str()),
                    tdg_id_origine: String::from(unwrapper(&self.tdg_id_origine).as_str()),
                    tdg_commune: String::from(unwrapper(&self.tdg_commune).as_str()),
                    fct_nom: String::from(unwrapper(&self.fct_nom).as_str()),
                    fct_prenom: String::from(unwrapper(&self.fct_prenom).as_str()),
                    tdg_adresse_full : contat_address(&address,&city)
        }
    }
}
 fn prospect_coordinates(){}
